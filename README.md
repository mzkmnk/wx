# wx

A CLI tool for managing Git worktrees and VSCode/Kiro workspaces.

[日本語](README.ja.md)

## Features

- Centralized management of multiple repositories via bare clone
- Automatic generation of worktrees and workspace files
- Accelerate parallel development setup

## Installation

### From crates.io

```bash
cargo install wx
```

### From source

```bash
git clone https://github.com/mzkmnk/wx.git
cd wx
cargo install --path .
```

## Usage

### Register repositories

```bash
wx register git@github.com:org/frontend.git
wx register git@github.com:org/backend.git
```

Registered repositories are bare cloned to `~/.wx/`.

### List registered repositories

```bash
wx list
```

### Create a workspace

```bash
cd ~/work
wx new feature-auth
```

Select repositories and branches interactively, then a `feature-auth/` directory will be created containing worktrees and a `.code-workspace` file.

## Data Location

```
~/.wx/
├── config.json        # Registered repositories
├── frontend.git/      # Bare repository
└── backend.git/       # Bare repository
```

## Development

```bash
# Build
cargo build

# Test
cargo test

# Release build
cargo build --release
```

## License

MIT License - Copyright (c) mzkmnk <mzk.mnk.dev@gmail.com>
