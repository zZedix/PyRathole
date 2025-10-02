# pyrathole

Python bindings for [rathole](https://github.com/rapiz1/rathole), a high-performance reverse tunnelling proxy. This package exposes Rathole's client and server entrypoints to Python via [PyO3](https://pyo3.rs/) and ships as a `cdylib` built with [maturin](https://github.com/PyO3/maturin).

Repository: <https://github.com/zZedix/PyRathole>

---

## Highlights

- 🔌 Launch Rathole **server** or **client** directly from Python.
- 🔁 Shares the same `*.toml` configuration format as the upstream Rust project.
- 🛑 Graceful shutdown support through `pyrathole.stop()`.
- 🪵 Reuses Rathole's tracing setup; configure logging with `RUST_LOG`.
- 🧪 Safe to embed inside long-running Python services thanks to built-in Tokio runtime management.

---

## Installation

### From PyPI (preferred)

```bash
pip install pyrathole
```

Requires Python 3.8+ (Linux, macOS, and Windows wheels are supported—builds rely on maturin's targets).

### From source

```bash
git clone https://github.com/zZedix/PyRathole.git
cd pyrathole
python -m pip install --upgrade pip
python -m pip install maturin
maturin develop  # builds and installs into the current virtualenv
```

For an isolated build without installing:

```bash
maturin build --release
# Produced wheels land in target/wheels/
```

---

## Quick Start

```python
import pyrathole

# Launch a rathole server using an existing configuration file
pyrathole.start_server("/path/to/server.toml")

# Launch a rathole client using an existing configuration file
pyrathole.start_client("/path/to/client.toml")

# Get rathole version
version = pyrathole.version()
print(f"Rathole version: {version}")
```

> **Important:** This package requires rathole to be installed on your system. Install it from [rathole releases](https://github.com/rapiz1/rathole/releases) or build from source.

---

## Configuration

pyrathole consumes the same configuration files as native Rathole. Refer to the upstream [configuration guide](https://github.com/rapiz1/rathole#configuration) for all options.

Example minimal server config (`server.toml`):

```toml
[server]
bind_addr = "0.0.0.0:2333"

[server.services.example]
type = "tcp"
bind_addr = "0.0.0.0:8080"
token = "super-secret"
```

Example matching client config (`client.toml`):

```toml
[client]
remote_addr = "your.server.com:2333"

[client.services.example]
type = "tcp"
local_addr = "127.0.0.1:8080"
token = "super-secret"
```

---

## Runtime & Logging

- pyrathole executes rathole as a subprocess, so logging is handled by the rathole binary itself.
- Set `RUST_LOG=debug` or similar environment variables to control rathole's logging verbosity.
- The Python wrapper provides error handling and status checking for the subprocess.

---

## Error Handling

The package provides proper error handling for common issues:

```python
import pyrathole

try:
    pyrathole.start_server("server.toml")
except RuntimeError as e:
    print(f"Failed to start server: {e}")
```

---

## Packaging & Release

### Building wheels

```bash
python -m pip install --upgrade pip maturin
maturin build --release  # optional --strip for smaller artifacts
```

Artifacts are written to `target/wheels/`. Inspect them with tools like `auditwheel` (Linux) or `delocate` (macOS) if you need to verify bundling.

### Publishing to PyPI

1. Ensure `~/.pypirc` is configured with your PyPI credentials (or use environment variables supported by maturin).
2. Build and upload in a single step:
   ```bash
   maturin publish --release
   ```
   Add `--username __token__` and `--password <pypi-token>` if not using `pypirc`.
3. Test the published artifact:
   ```bash
   python -m pip install --upgrade pyrathole
   python -c "import pyrathole; print('pyrathole', pyrathole.__doc__ is not None)"
   ```

### GitHub Release Checklist

1. Update `Cargo.toml` and `pyproject.toml` versions (keep them in sync).
2. Regenerate wheels via `maturin build --release`.
3. Commit and tag the release:
   ```bash
   git add .
   git commit -m "Release vX.Y.Z"
   git tag vX.Y.Z
   git push origin main --tags
   ```
4. Draft a GitHub release on <https://github.com/zZedix/PyRathole/releases/new> summarising changes and attach wheel artifacts from `target/wheels/` if you distribute binaries outside of PyPI.
5. Publish PyPI release (`maturin publish`) and update the release notes with installation instructions.

---

## Local Development

- Format & lint (requires Rust toolchain):
  ```bash
  cargo fmt
  cargo clippy --all-targets --all-features
  ```
- Run Rust tests:
  ```bash
  cargo test
  ```
- Validate the Python module in editable mode:
  ```bash
  maturin develop
  python -m pip install pytest
  PYTHONPATH=. pytest tests/python
  ```

> Note: The Python test suite is optional; adapt the commands to your project's structure.

---

## Troubleshooting

| Symptom | Possible Cause | Suggested Fix |
|---------|----------------|---------------|
| `RuntimeError: Failed to start rathole client/server` | rathole binary not found in PATH | Install rathole from [releases](https://github.com/rapiz1/rathole/releases) or build from source |
| `RuntimeError: Rathole client/server failed` | Invalid configuration file or network issues | Check your TOML configuration and network connectivity |
| Build fails with `rustc: command not found` | Rust toolchain missing | Install Rust via [rustup.rs](https://rustup.rs/) |
| `RuntimeError: Failed to get rathole version` | rathole binary not installed or not in PATH | Ensure rathole is properly installed and accessible |

---

## Contributing

Issues and pull requests are welcome. Open a ticket at <https://github.com/zZedix/PyRathole/issues> with a clear description of the problem or proposed enhancement. For code changes, please run the checks in the "Local Development" section before submitting a PR.

---

## License

pyrathole inherits Rathole's licensing (Apache-2.0). See `LICENSE` in the upstream repository for details.

---

## Useful Links

- Project repository: <https://github.com/zZedix/PyRathole>
- Rathole upstream: <https://github.com/rapiz1/rathole>
- PyO3 user guide: <https://pyo3.rs/>
- maturin documentation: <https://maturin.rs/>
