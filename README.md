# Rimc

[![CI Status](https://github.com/AUTOM77/Rimc/workflows/ci/badge.svg)](https://github.com/AUTOM77/Rimc/actions?query=workflow:ci)
[![Code Size](https://img.shields.io/github/languages/code-size/AUTOM77/Rimc)](.)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE)
[![Open Issues](https://img.shields.io/github/issues/AUTOM77/Rimc)](https://github.com/AUTOM77/Rimc/issues)

> Rimc, a Rust based Multi-Modal Hyper Caption Tool in Parallel

### Usage

1. For a single image/video, it will returns a `*.txt`.

```bash
rimc -f ${file_path} -c `config.toml`
```

2. For a batch of image/video list, it will returns a list of `*.txt`.

```bash
rimc -d ${dir_path} -c `config.toml`
```

### Config

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