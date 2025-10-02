# PyRathole

[![PyPI version](https://badge.fury.io/py/pyrathole.svg)](https://badge.fury.io/py/pyrathole)
[![Python 3.8+](https://img.shields.io/badge/python-3.8+-blue.svg)](https://www.python.org/downloads/)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-yellow.svg)](https://opensource.org/licenses/Apache-2.0)

Python bindings for [rathole](https://github.com/rapiz1/rathole), a high-performance reverse tunnelling proxy.

## 🚀 Quick Start

### Installation

```bash
pip install pyrathole
```

### Basic Usage

```python
import pyrathole

# Start rathole server
pyrathole.start_server("/path/to/server.toml")

# Start rathole client  
pyrathole.start_client("/path/to/client.toml")

# Get rathole version
version = pyrathole.version()
print(f"Rathole version: {version}")
```

> **Note**: This package requires rathole to be installed on your system. Download from [rathole releases](https://github.com/rapiz1/rathole/releases).

## 📋 Requirements

- Python 3.8+
- rathole binary installed on your system
- Valid rathole configuration files

## 🔧 Installing rathole Binary

Before using PyRathole, you need to install the rathole binary on your system:

### Option 1: Download Pre-built Binaries (Recommended)

1. Go to [rathole releases](https://github.com/rapiz1/rathole/releases)
2. Download the appropriate binary for your system:
   - **Linux**: `rathole-x86_64-unknown-linux-gnu.tar.gz`
   - **macOS**: `rathole-x86_64-apple-darwin.tar.gz` or `rathole-aarch64-apple-darwin.tar.gz`
   - **Windows**: `rathole-x86_64-pc-windows-gnu.zip`

3. Extract and install:
   ```bash
   # Linux/macOS
   tar -xzf rathole-*.tar.gz
   sudo mv rathole /usr/local/bin/
   
   # Windows
   # Extract the zip file and add the directory to your PATH
   ```

### Option 2: Install via Package Manager

#### macOS (Homebrew)
```bash
brew install rathole
```

#### Arch Linux
```bash
sudo pacman -S rathole
```

#### Ubuntu/Debian (via cargo)
```bash
cargo install rathole
```

### Option 3: Build from Source

```bash
git clone https://github.com/rapiz1/rathole.git
cd rathole
cargo build --release
sudo cp target/release/rathole /usr/local/bin/
```

### Verify Installation

```bash
rathole --version
```

This should output the rathole version number.

## ⚙️ Configuration

PyRathole uses the same configuration format as native rathole. See the [rathole documentation](https://github.com/rapiz1/rathole#configuration) for detailed configuration options.

### Example Server Config (`server.toml`)

```toml
[server]
bind_addr = "0.0.0.0:2333"

[server.services.web]
type = "tcp"
bind_addr = "0.0.0.0:8080"
token = "your-secret-token"
```

### Example Client Config (`client.toml`)

```toml
[client]
remote_addr = "your.server.com:2333"

[client.services.web]
type = "tcp"
local_addr = "127.0.0.1:8080"
token = "your-secret-token"
```

## 🔧 API Reference

### Functions

- `start_server(config_path: str)` - Start rathole server with given config file
- `start_client(config_path: str)` - Start rathole client with given config file  
- `version() -> str` - Get installed rathole version

### Error Handling

```python
import pyrathole

try:
    pyrathole.start_server("server.toml")
except RuntimeError as e:
    print(f"Failed to start server: {e}")
```

## 🐛 Troubleshooting

| Error | Cause | Solution |
|-------|-------|----------|
| `RuntimeError: Failed to start rathole` | rathole not found in PATH | Install rathole binary and ensure it's in your PATH |
| `RuntimeError: Rathole failed` | Invalid config or network issues | Check configuration file and network connectivity |
| `RuntimeError: Failed to get version` | rathole not installed | Ensure rathole is properly installed and accessible |
| `command not found: rathole` | rathole not in PATH | Add rathole to your system PATH or install it properly |
| Permission denied | Insufficient permissions | Use `sudo` for system-wide installation or install to user directory |

### Common Installation Issues

**Linux/macOS:**
```bash
# Check if rathole is installed
which rathole

# If not found, add to PATH
export PATH="/usr/local/bin:$PATH"
```

**Windows:**
- Ensure rathole.exe is in a directory that's in your PATH
- Or add the rathole directory to your system PATH environment variable

## 📚 Documentation

- [Rathole Documentation](https://github.com/rapiz1/rathole)
- [PyPI Package](https://pypi.org/project/pyrathole/)
- [GitHub Repository](https://github.com/zZedix/PyRathole)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [rathole](https://github.com/rapiz1/rathole) - The amazing reverse tunneling proxy
- [PyO3](https://pyo3.rs/) - Rust-Python bindings
- [maturin](https://maturin.rs/) - Build tool for Python extensions
