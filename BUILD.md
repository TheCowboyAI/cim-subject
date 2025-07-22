<!-- Copyright (c) 2025 Cowboy AI, LLC. -->

# Building CIM-Subject

This document provides comprehensive instructions for building and developing the CIM-Subject module.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Build Methods](#build-methods)
  - [Using Nix (Recommended)](#using-nix-recommended)
  - [Using Cargo](#using-cargo)
  - [Using NixOS Containers](#using-nixos-containers)
- [Development Environment](#development-environment)
- [Running Tests](#running-tests)
- [Running Examples](#running-examples)
- [Building Documentation](#building-documentation)
- [Performance Profiling](#performance-profiling)
- [Release Builds](#release-builds)
- [Troubleshooting](#troubleshooting)

## Prerequisites

### Required

- **Rust**: 1.75.0 or later
- **Git**: For cloning the repository

### Optional

- **Nix**: For reproducible builds (recommended)
- **NATS Server**: For integration examples
- **NixOS**: For container deployment
- **extra-container**: For easy container management

## Quick Start

### With Nix (Recommended)

```bash
# Clone the repository
git clone https://github.com/thecowboyai/cim-subject.git
cd cim-subject

# Enter development environment
nix develop

# Build and test
cargo build
cargo test
```

### Without Nix

```bash
# Clone the repository
git clone https://github.com/thecowboyai/cim-subject.git
cd cim-subject

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build and test
cargo build
cargo test
```

## Build Methods

### Using Nix (Recommended)

Nix provides a reproducible development environment with all dependencies.

#### Development Shell

```bash
# Enter development shell with all tools
nix develop

# Or run commands directly
nix develop -c cargo build
nix develop -c cargo test
```

#### Building with Nix

```bash
# Build the package
nix build

# Run checks
nix flake check

# Build and run an example
nix run .#basic-routing
nix run .#mortgage-routing
```

#### Available Nix Apps

- `basic-routing` - Basic message routing example
- `correlation-tracking` - Message correlation example
- `mortgage-routing` - Mortgage lending routing example
- `document-validation` - Document validation example
- `rate-shopping` - Rate shopping example

### Using Cargo

Standard Rust build process:

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with all features
cargo build --all-features

# Check without building
cargo check
```

### Using NixOS Containers

Deploy using NixOS containers with extra-container:

```nix
# container.nix
{ config, pkgs, ... }:
{
  containers.cim-subject = {
    autoStart = true;
    privateNetwork = true;
    hostAddress = "192.168.100.1";
    localAddress = "192.168.100.2";
    
    config = { config, pkgs, ... }: {
      imports = [
        # Import the flake's NixOS module
        (builtins.getFlake "github:thecowboyai/cim-subject").nixosModules.default
      ];
      
      services.cim-subject = {
        enable = true;
        natsUrl = "192.168.100.1:4222";
        logLevel = "info";
      };
      
      # Minimal container configuration
      system.stateVersion = "23.11";
      networking.firewall.enable = false;
    };
  };
}
```

Deploy with extra-container:

```bash
# Single container deployment
sudo extra-container create --start < container.nix

# Or use the flake output
sudo extra-container create --flake .#container

# Multi-container deployment with NATS
sudo extra-container create --start < compose.nix

# Using nixos-container directly
sudo nixos-container create cim-subject --config-file container.nix
sudo nixos-container start cim-subject

# Management commands
sudo nixos-container status cim-subject
sudo nixos-container run cim-subject -- journalctl -u cim-subject
sudo nixos-container root-shell cim-subject

# Clean up
sudo extra-container destroy cim-subject
```

#### Container Networking

The provided configurations use private networking:
- NATS: `10.233.1.0/24`
- CIM-Subject: `10.233.2.0/24`
- Future services: `10.233.3.0/24`, etc.

Adjust these ranges to match your infrastructure.

## Development Environment

### IDE Setup

#### VS Code

Install recommended extensions:
- rust-analyzer
- Even Better TOML
- crates

Settings (`.vscode/settings.json`):
```json
{
    "rust-analyzer.cargo.features": "all",
    "rust-analyzer.checkOnSave.command": "clippy"
}
```

#### IntelliJ IDEA / CLion

Install the Rust plugin and configure:
- Set Rust toolchain path
- Enable clippy for on-save checks
- Configure cargo features

### Environment Variables

```bash
# Enable debug logging
export RUST_LOG=debug

# Enable backtrace
export RUST_BACKTRACE=1

# NATS connection (for examples)
export NATS_URL=localhost:4222
```

## Running Tests

### Unit Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_pattern_matching

# Run tests in specific module
cargo test pattern::
```

### Integration Tests

```bash
# Run integration tests (requires NATS)
nats-server -js &
cargo test --test '*' -- --test-threads=1
```

### Test Coverage

```bash
# Using tarpaulin
cargo tarpaulin --out Html --output-dir coverage

# Using llvm-cov (with nightly)
cargo +nightly llvm-cov --html
```

### Property-Based Tests

```bash
# Run property tests (uses proptest)
cargo test --features proptest
```

## Running Examples

### Basic Examples

```bash
# List all examples
cargo run --example

# Basic routing
cargo run --example basic_routing

# Correlation tracking
cargo run --example correlation_tracking

# Subject validation
cargo run --example subject_validation

# Permissions
cargo run --example permissions

# Algebra operations
cargo run --example algebra_operations

# Translation
cargo run --example translation
```

### NATS Integration Example

```bash
# Start NATS server
nats-server -js

# Run NATS integration example
cargo run --example nats_integration
```

### Domain-Specific Examples

```bash
# Mortgage lending routing
cargo run --example mortgage_lending_routing

# Document validation workflow
cargo run --example document_validation

# Multi-lender rate shopping
cargo run --example rate_shopping
```

## Building Documentation

### API Documentation

```bash
# Build documentation
cargo doc

# Build and open in browser
cargo doc --open

# Include dependencies
cargo doc --all-features --no-deps

# Build with private items
cargo doc --document-private-items
```

### User Documentation

```bash
# Using mdbook (if installed)
mdbook build doc/
mdbook serve doc/
```

## Performance Profiling

### Benchmarks

```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench pattern_matching

# Save baseline
cargo bench -- --save-baseline main
```

### Profiling

Using `perf` on Linux:

```bash
# Build with debug symbols
cargo build --release

# Profile
perf record --call-graph=dwarf target/release/examples/basic_routing
perf report
```

Using Instruments on macOS:

```bash
# Build with debug symbols
cargo build --release

# Profile with Instruments
instruments -t "Time Profiler" target/release/examples/basic_routing
```

### Memory Profiling

Using Valgrind:

```bash
cargo build --release
valgrind --tool=massif target/release/examples/basic_routing
ms_print massif.out.*
```

## Release Builds

### Optimization Levels

In `Cargo.toml`:

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

### Cross-Compilation

```bash
# Add target
rustup target add x86_64-unknown-linux-musl

# Build for target
cargo build --release --target x86_64-unknown-linux-musl
```

### Creating a Release

```bash
# Update version in Cargo.toml
# Update CHANGELOG.md

# Create git tag
git tag -a v0.5.0 -m "Release version 0.5.0"
git push origin v0.5.0

# Publish to crates.io
cargo publish --dry-run
cargo publish
```

## Troubleshooting

### Common Issues

#### Compilation Errors

```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Check for outdated dependencies
cargo outdated
```

#### Linking Errors

On macOS:
```bash
export MACOSX_DEPLOYMENT_TARGET=10.14
```

On Linux with OpenSSL issues:
```bash
sudo apt-get install pkg-config libssl-dev
```

#### Test Failures

```bash
# Run tests sequentially
cargo test -- --test-threads=1

# Increase test timeout
RUST_TEST_THREADS=1 cargo test -- --timeout 300
```

### Debug Builds

Enable additional debug information:

```toml
[profile.dev]
debug = 2
overflow-checks = true
```

### Logging

Enable detailed logging:

```bash
# Module-specific logging
RUST_LOG=cim_subject=debug cargo run --example basic_routing

# All debug logs
RUST_LOG=debug cargo test

# Trace level for specific module
RUST_LOG=cim_subject::pattern=trace cargo test pattern
```

## CI/CD Integration

### GitHub Actions

See `.github/workflows/ci.yml` for the complete CI pipeline.

### Pre-commit Hooks

Install pre-commit hooks:

```bash
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/sh
cargo fmt --check
cargo clippy -- -D warnings
cargo test
EOF

chmod +x .git/hooks/pre-commit
```

## Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Guide](https://doc.rust-lang.org/cargo/)
- [API Documentation](https://docs.rs/cim-subject)
- [NATS Documentation](https://docs.nats.io/)

For more information, see the [main README](README.md) or the [module documentation](MODULE.md).