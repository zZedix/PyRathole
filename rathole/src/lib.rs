mod config;
mod config_watcher;
mod constants;
mod helper;
mod multi_map;
mod protocol;
mod transport;

pub use config::Config;
pub use constants::UDP_BUFFER_SIZE;

use anyhow::{anyhow, bail, Result};
use lazy_static::lazy_static;
use std::{
    error::Error,
    path::PathBuf,
    sync::{Mutex, Once},
};
use tokio::sync::{broadcast, mpsc};
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
use client::run_client;

#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
use server::run_server;

use config_watcher::{ConfigChange, ConfigWatcherHandle};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoiseCurve {
    X25519,
    X448,
}

pub const DEFAULT_NOISE_CURVE: NoiseCurve = NoiseCurve::X25519;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartMode {
    Auto,
    Client,
    Server,
}

#[derive(Clone, Debug)]
pub struct RatholeOptions {
    pub config_path: PathBuf,
    pub mode: StartMode,
}

impl RatholeOptions {
    pub fn from_path<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            config_path: path.into(),
            mode: StartMode::Auto,
        }
    }

    pub fn client<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            config_path: path.into(),
            mode: StartMode::Client,
        }
    }

    pub fn server<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            config_path: path.into(),
            mode: StartMode::Server,
        }
    }
}

lazy_static! {
    static ref GLOBAL_SHUTDOWN: Mutex<Option<broadcast::Sender<bool>>> = Mutex::new(None);
}

static INIT_TRACING: Once = Once::new();

pub fn init_logging() {
    INIT_TRACING.call_once(|| {
        #[cfg(feature = "console")]
        {
            console_subscriber::init();
            tracing::info!("console_subscriber enabled");
        }

        #[cfg(not(feature = "console"))]
        {
            let is_atty = atty::is(atty::Stream::Stdout);
            let level = "info";
            tracing_subscriber::fmt()
                .with_env_filter(
                    EnvFilter::try_from_default_env()
                        .unwrap_or_else(|_| EnvFilter::from(level)),
                )
                .with_ansi(is_atty)
                .init();
        }
    });
}

pub fn start(config_path: &str) -> Result<(), Box<dyn Error>> {
    start_with_options(RatholeOptions::from_path(config_path))
}

pub fn start_client(config_path: &str) -> Result<(), Box<dyn Error>> {
    start_with_options(RatholeOptions::client(config_path))
}

pub fn start_server(config_path: &str) -> Result<(), Box<dyn Error>> {
    start_with_options(RatholeOptions::server(config_path))
}

pub fn start_with_options(options: RatholeOptions) -> Result<(), Box<dyn Error>> {
    init_logging();

    {
        let mut guard = GLOBAL_SHUTDOWN.lock().unwrap();
        if guard.is_some() {
            return Err(anyhow!("rathole instance already running").into());
        }
    }

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    let (shutdown_tx, shutdown_rx) = broadcast::channel::<bool>(1);

    {
        let mut guard = GLOBAL_SHUTDOWN.lock().unwrap();
        *guard = Some(shutdown_tx.clone());
    }

    let result = runtime.block_on(async { run_with_shutdown(options, shutdown_rx).await });

    {
        let mut guard = GLOBAL_SHUTDOWN.lock().unwrap();
        guard.take();
    }

    drop(runtime);

    result.map_err(|e| e.into())
}

pub fn stop() -> Result<(), Box<dyn Error>> {
    let shutdown = {
        let mut guard = GLOBAL_SHUTDOWN.lock().unwrap();
        guard.take()
    };

    if let Some(tx) = shutdown {
        let _ = tx.send(true);
        Ok(())
    } else {
        Err(anyhow!("no running rathole instance").into())
    }
}

pub fn generate_noise_keypair(curve: Option<NoiseCurve>) -> Result<(String, String)> {
    #[cfg(feature = "noise")]
    {
        let curve = curve.unwrap_or(DEFAULT_NOISE_CURVE);
        let builder = snowstorm::Builder::new(
            format!(
                "Noise_KK_{}_ChaChaPoly_BLAKE2s",
                match curve {
                    NoiseCurve::X25519 => "25519",
                    NoiseCurve::X448 => "448",
                }
            )
            .parse()?,
        );
        let keypair = builder.generate_keypair()?;

        Ok((
            base64::encode(keypair.private),
            base64::encode(keypair.public),
        ))
    }

    #[cfg(not(feature = "noise"))]
    {
        helper::feature_not_compile("noise")
    }
}

pub async fn run_with_shutdown(
    options: RatholeOptions,
    shutdown_rx: broadcast::Receiver<bool>,
) -> Result<()> {
    let RatholeOptions { config_path, mode } = options;

    fdlimit::raise_fd_limit();

    let mut cfg_watcher =
        ConfigWatcherHandle::new(config_path.as_path(), shutdown_rx).await?;

    let (instance_shutdown_tx, _) = broadcast::channel::<bool>(1);
    let mut last_instance: Option<(
        tokio::task::JoinHandle<Result<()>>,
        mpsc::Sender<ConfigChange>,
    )> = None;

    while let Some(event) = cfg_watcher.event_rx.recv().await {
        match event {
            ConfigChange::General(config) => {
                if let Some((handle, _)) = last_instance.take() {
                    info!("General configuration change detected. Restarting...");
                    let _ = instance_shutdown_tx.send(true);
                    handle.await??;
                }

                debug!("{:?}", config);

                let (service_update_tx, service_update_rx) = mpsc::channel(1024);

                let join_handle = tokio::spawn(run_instance(
                    *config,
                    mode,
                    instance_shutdown_tx.subscribe(),
                    service_update_rx,
                ));

                last_instance = Some((join_handle, service_update_tx));
            }
            other => {
                info!("Service change detected. {:?}", other);
                if let Some((_, service_update_tx)) = &last_instance {
                    let _ = service_update_tx.send(other).await;
                }
            }
        }
    }

    let _ = instance_shutdown_tx.send(true);

    if let Some((handle, _)) = last_instance {
        handle.await??;
    }

    Ok(())
}

async fn run_instance(
    config: Config,
    mode: StartMode,
    shutdown_rx: broadcast::Receiver<bool>,
    service_update: mpsc::Receiver<ConfigChange>,
) -> Result<()> {
    match determine_run_mode(&config, mode) {
        RunMode::Undetermine => {
            bail!("Cannot determine running as a server or a client");
        }
        RunMode::Client => {
            #[cfg(not(feature = "client"))]
            {
                let _ = (config, shutdown_rx, service_update);
                helper::feature_not_compile("client");
            }
            #[cfg(feature = "client")]
            {
                run_client(config, shutdown_rx, service_update).await
            }
        }
        RunMode::Server => {
            #[cfg(not(feature = "server"))]
            {
                let _ = (config, shutdown_rx, service_update);
                helper::feature_not_compile("server");
            }
            #[cfg(feature = "server")]
            {
                run_server(config, shutdown_rx, service_update).await
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum RunMode {
    Server,
    Client,
    Undetermine,
}

fn determine_run_mode(config: &Config, mode: StartMode) -> RunMode {
    use RunMode::*;

    match mode {
        StartMode::Client => Client,
        StartMode::Server => Server,
        StartMode::Auto => {
            let has_client = config.client.is_some();
            let has_server = config.server.is_some();

            match (has_server, has_client) {
                (true, false) => Server,
                (false, true) => Client,
                _ => Undetermine,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::*;

    #[test]
    fn test_determine_run_mode() {
        use RunMode::*;

        struct T {
            cfg_s: bool,
            cfg_c: bool,
            mode: StartMode,
            expected: RunMode,
        }

        let tests = [
            T {
                cfg_s: false,
                cfg_c: false,
                mode: StartMode::Auto,
                expected: Undetermine,
            },
            T {
                cfg_s: true,
                cfg_c: false,
                mode: StartMode::Auto,
                expected: Server,
            },
            T {
                cfg_s: false,
                cfg_c: true,
                mode: StartMode::Auto,
                expected: Client,
            },
            T {
                cfg_s: true,
                cfg_c: true,
                mode: StartMode::Auto,
                expected: Undetermine,
            },
            T {
                cfg_s: true,
                cfg_c: true,
                mode: StartMode::Server,
                expected: Server,
            },
            T {
                cfg_s: true,
                cfg_c: true,
                mode: StartMode::Client,
                expected: Client,
            },
        ];

        for t in &tests {
            let config = Config {
                server: if t.cfg_s {
                    Some(ServerConfig::default())
                } else {
                    None
                },
                client: if t.cfg_c {
                    Some(ClientConfig::default())
                } else {
                    None
                },
            };

            assert_eq!(determine_run_mode(&config, t.mode), t.expected);
        }
    }
}
