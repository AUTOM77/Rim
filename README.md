# Rim

[![CI Status](https://github.com/AUTOM77/Rim/workflows/ci/badge.svg)](https://github.com/AUTOM77/Rim/actions?query=workflow:ci)
[![Code Size](https://img.shields.io/github/languages/code-size/AUTOM77/Rim)](.)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE)
[![Open Issues](https://img.shields.io/github/issues/AUTOM77/Rim)](https://github.com/AUTOM77/Rim/issues)

> Rim, a Rust based Multi-Modal Hyper Caption Tool in Parallel

### Usage

1. **Single Image/Video Captioning:**

```bash
rim -f ${file_path} -c `config.toml`
```
Rim generates a `*.txt` file containing the caption for a single image or video.

2. **Batch Image/Video Captioning:**

```bash
rim -d ${dir_path} -c `config.toml`
```

For a directory of images or videos, `Rim` generates a corresponding list of `*.txt` caption files.

3. Sample `config.toml` can be found in [config.toml](./config.toml)

### Config

Creating a Sample Configuration (Unix):

```bash
cat <<EOF | tee config.toml
[prompt]
value = "Caption this media"

[gemini]
keys = [
    "AIza00000000000000000000000000000000000",
    "AIza00000000000000000000000000000000001",
]

[gpt4v]
keys = [
    "sk-00000000000000000000000000000000",
    "sk-00000000000000000000000000000001",
]
EOF
```

### Compile from source

```bash
sudo apt install -y --no-install-recommends pkg-config yasm nasm musl-dev clang llvm

curl -fsSL https://sh.rustup.rs | sh -s -- -y
rustup update nightly && rustup default nightly
rustup component add rust-std-x86_64-unknown-linux-musl
cargo build --release
```