# Rim

[![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/AUTOM77/Rim/ci.yml)](https://github.com/AUTOM77/Rim/actions)
[![GitHub license](https://img.shields.io/github/license/AUTOM77/Rim)](./LICENSE)
[![GitHub contributors](https://img.shields.io/github/contributors/AUTOM77/Rim)](https://github.com/AUTOM77/Rim/graphs/contributors)
[![GitHub commit activity (branch)](https://img.shields.io/github/commit-activity/m/AUTOM77/Rim)](https://github.com/AUTOM77/Rim/commits)
[![GitHub top language](https://img.shields.io/github/languages/top/AUTOM77/Rim?logo=rust&label=)](./rim-cli/Cargo.toml#L4)
[![Open Issues](https://img.shields.io/github/issues/AUTOM77/Rim)](https://github.com/AUTOM77/Rim/issues)
[![Code Size](https://img.shields.io/github/languages/code-size/AUTOM77/Rim)](.)
[![GitHub all releases](https://img.shields.io/github/downloads/AUTOM77/Rim/total?logo=github)](https://github.com/AUTOM77/Rim/releases)  
[![GitHub release (with filter)](https://img.shields.io/github/v/release/AUTOM77/Rim?logo=github)](https://github.com/AUTOM77/Rim/releases)

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

```dash
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

### Nightly Build

```dash
curl -fsSL https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"
rustup update nightly && rustup default nightly
cargo build --release

./target/release/rim "assets/images" -c config.toml
```