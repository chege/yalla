[![CI](https://github.com/chege/yalla/actions/workflows/ci.yaml/badge.svg)](https://github.com/chege/yalla/actions/workflows/ci.yaml)

# Yalla

> A fast, configurable CLI task runner for teams and solo developers

Yalla is a modern CLI runner that executes commands defined in TOML configuration files. Streamline your development
workflow with a simple, declarative approach to task management—no more hunting through makefiles or remembering complex
command sequences.

## Quick Start

1. **Install Yalla**:
   ```bash
   cargo install --git https://github.com/chege/yalla
   ```

2. **Create a `Yallafile`** in your project root:
   ```toml
   [tools.test]
   description = "Run all tests with coverage"
   command = "cargo test --all-features"
   
   [tools.lint]
   description = "Check code formatting and style"
   command = "cargo clippy -- -D warnings"
   
   [tools.dev]
   description = "Start development server with hot reload"
   command = "cargo watch -x run"
   ```

3. **Run your tasks**:
   ```bash
   yalla tools test    # Run tests
   yalla tools lint    # Run linting
   yalla tools dev     # Start dev server
   ```

## Installation Options

### From GitHub (Recommended)

```bash
cargo install --git https://github.com/chege/yalla
```

### From Source

```bash
git clone https://github.com/chege/yalla.git
cd yalla
cargo install --path .
```

### Development Build

```bash
git clone https://github.com/chege/yalla.git
cd yalla
cargo build --release
./target/release/yalla --help
```

## Configuration Examples

### Basic Commands

```toml
[tools.build]
description = "Build the project in release mode"
command = "cargo build --release"

[tools.clean]
description = "Clean build artifacts"
command = "cargo clean"
```

### Commands with Arguments

```toml
[tools.format]
description = "Format code (use --check for dry run)"
command = "cargo fmt"

[tools.benchmark]
description = "Run performance benchmarks"
command = "cargo bench --all-features"
```

### Environment-Specific Commands

```toml
[tools.deploy-staging]
description = "Deploy to staging environment"
command = "kubectl apply -f k8s/staging/"

[tools.deploy-prod]
description = "Deploy to production environment"
command = "kubectl apply -f k8s/production/"
```

## Command Reference

### List Available Tools

```bash
yalla tools --list
# or
yalla tools ls
```

### Run a Specific Tool

```bash
yalla tools <tool-name>
```

### Get Help

```bash
yalla --help
yalla tools --help
```

## Advanced Features

### Working Directory Support

```toml
[tools.frontend-build]
description = "Build frontend assets"
command = "npm run build"
working_dir = "./frontend"
```

### Environment Variables

```toml
[tools.test-integration]
description = "Run integration tests"
command = "cargo test integration_tests"
env = { RUST_LOG = "debug", DATABASE_URL = "postgres://localhost/test_db" }
```

### Development Commands

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Check formatting
cargo fmt --check

# Run clippy lints
cargo clippy -- -D warnings

# Build release binary
cargo build --release
```
