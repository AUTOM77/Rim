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

> Rim, a Rust based Multi-Modal Hyper Caption Tool in Parallel, v3.0 released!

## Features

* [x] support `Universal image/video media mixed caption task`
* [x] support `OpenAI Models in Azure Platform, GPT-4o, GPT-4v`
* [x] support `Gemini Model in Google Cloud Platform, Gemini-1.5-flash, Gemini-1.5-pro`
* [x] support `Multi-Prompt with seperate naming space`
* [x] support `Optional Service Selection`
* [x] support `QPS config, default is 20 in parallel`
* [x] support `Limit config, default is first 100 jobs`
* [x] support `Seperate saving path for $MODEL/$PROMPT/$File.txt`


### Usage

> [!TIP]
> rim `assets/images/1.png` -c config.toml --limit 100 --qps 20

For a single key on single project, we recommend using `rim ${path} -c config.toml --limit 360`.

<details>
  <summary>Old Usage</summary>

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
</details>

---

1. Rim will now generates a folder called `xxx_cap` contains `*.txt` caption files.
2. Sample `config.toml` can be found in [config.toml](./config.toml)

### Config

Creating a Sample Configuration (Unix):

```dash
cat <<EOF | tee config.toml
[[prompt]]
name = "simple"
value = "Caption this video."

[[prompt]]
name = "example"
value = "Provide a brief summary of the video content focusing on key themes and messages."

[azure]
api = [
    ['https://closedAI-1.openai.azure.com', 'sk-00000000000000000000000000000000', 'gpt-4o'],
    ['https://closedAI-2.openai.azure.com', 'sk-00000000000000000000000000000001', 'gpt-4v']
]

[gemini]
api = [
    ['https://generativelanguage.googleapis.com', 'AIza00000000000000000000000000000000000', 'gemini-1.5-flash-latest'],
    ['https://generativelanguage.googleapis.com', 'AIza00000000000000000000000000000000001', 'gemini-1.5-pro-latest'],
]
EOF
```

### Nightly Build

```sh
curl -fsSL https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"
rustup update nightly && rustup default nightly

cargo build --release
./target/release/rim "assets/images" -c config.toml
```

### Nightly Build with mirror
```sh
curl -fsSL https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"
echo """
[source.crates-io]
replace-with = 'mirror'

[source.mirror]
registry = 'sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/'
""" | tee ${CARGO_HOME:-$HOME/.cargo}/config.toml
rustup update nightly && rustup default nightly

cargo build --release
./target/release/rim "assets/images" -c config.toml
```