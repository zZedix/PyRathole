# Contributing to PyRathole

Thank you for your interest in contributing to PyRathole! This document provides guidelines and information for contributors.

## Getting Started

### Prerequisites

- Python 3.8+
- Rust toolchain (install via [rustup](https://rustup.rs/))
- Git

### Development Setup

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/zZedix/PyRathole.git
   cd PyRathole
   ```

3. Install development dependencies:
   ```bash
   pip install maturin
   ```

4. Build the project in development mode:
   ```bash
   maturin develop
   ```

## Development Workflow

### Making Changes

1. Create a new branch for your feature:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes to the Rust code in `src/lib.rs`
3. Update tests if necessary
4. Update documentation if needed

### Testing

Run the Rust tests:
```bash
cargo test
```

Test the Python module:
```bash
python -c "import pyrathole; print('Import successful')"
```

### Building

Build the project:
```bash
maturin build --release
```

### Code Style

- Follow Rust conventions (run `cargo fmt` and `cargo clippy`)
- Use meaningful commit messages
- Update documentation for new features

## Submitting Changes

### Pull Request Process

1. Ensure your code follows the project's style guidelines
2. Add tests for new functionality
3. Update documentation as needed
4. Commit your changes with descriptive messages
5. Push your branch to your fork
6. Submit a pull request with a clear description

### Pull Request Guidelines

- Use a clear, descriptive title
- Provide a detailed description of changes
- Reference any related issues
- Ensure all tests pass
- Update documentation if needed

## Issue Reporting

When reporting issues, please include:

- Python version
- Operating system
- Steps to reproduce
- Expected vs actual behavior
- Any error messages

## Code of Conduct

This project follows the [Contributor Covenant](https://www.contributor-covenant.org/) Code of Conduct.

## License

By contributing to PyRathole, you agree that your contributions will be licensed under the Apache License 2.0.

