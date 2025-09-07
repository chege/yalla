[![CI](https://github.com/chege/yalla/actions/workflows/ci.yaml/badge.svg)](https://github.com/chege/yalla/actions/workflows/ci.yaml)

# Yalla

Yalla is a configurable CLI runner designed to execute commands defined in a TOML configuration file. It provides a
flexible and efficient way to manage and run processes through a simple, declarative interface.

## Features

- Simple process handling
- Robust exit status management
- Lightweight and easy to use

## Usage

Define your commands in a `Yallafile` using TOML syntax, for example:

```toml
[tools.ls]
description = "Shows some stuff"
command = "ls"

[tools.echo]
command = "echo"
```

Run commands using Yalla:

```bash
yalla tools ls
yalla tools echo
```

## Getting Started for Developers

Clone the repository:

```bash
git clone https://github.com/chege/yalla.git
cd yalla
```

Build the project:

```bash
cargo build --release
```

Run tests:

```bash
cargo test
```
