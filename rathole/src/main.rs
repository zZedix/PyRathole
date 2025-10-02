mod cli;

use anyhow::{anyhow, Result};
use clap::Parser;
use cli::{Cli, KeypairType};
use rathole::{
    generate_noise_keypair, init_logging, run_with_shutdown, NoiseCurve, RatholeOptions, StartMode,
};
use tokio::{signal, sync::broadcast};

impl From<KeypairType> for NoiseCurve {
    fn from(value: KeypairType) -> Self {
        match value {
            KeypairType::X25519 => NoiseCurve::X25519,
            KeypairType::X448 => NoiseCurve::X448,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    if let Some(genkey) = args.genkey {
        let curve = genkey.unwrap_or(KeypairType::X25519);
        let (private_key, public_key) = generate_noise_keypair(Some(curve.into()))?;
        println!("Private Key:\n{}\n", private_key);
        println!("Public Key:\n{}", public_key);
        return Ok(());
    }

    let config_path = args
        .config_path
        .clone()
        .ok_or_else(|| anyhow!("configuration path is required"))?;

    let mut options = RatholeOptions::from_path(config_path);
    options.mode = if args.client && args.server {
        StartMode::Auto
    } else if args.client {
        StartMode::Client
    } else if args.server {
        StartMode::Server
    } else {
        StartMode::Auto
    };

    let (shutdown_tx, shutdown_rx) = broadcast::channel::<bool>(1);
    let shutdown_signal = shutdown_tx.clone();
    tokio::spawn(async move {
        if let Err(e) = signal::ctrl_c().await {
            panic!("Failed to listen for the ctrl-c signal: {:?}", e);
        }

        if let Err(e) = shutdown_signal.send(true) {
            panic!("Failed to send shutdown signal: {:?}", e);
        }
    });

    init_logging();
    run_with_shutdown(options, shutdown_rx).await
}
