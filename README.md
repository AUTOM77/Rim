# Rimc


[![CI Status](https://github.com/AUTOM77/Rimc/workflows/ci/badge.svg)](https://github.com/AUTOM77/Rimc/actions?query=workflow:ci)
[![Code Size](https://img.shields.io/github/languages/code-size/AUTOM77/Rimc)](.)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE)
[![Open Issues](https://img.shields.io/github/issues/AUTOM77/Rimc)](https://github.com/AUTOM77/Rimc/issues)

> Rimc, a Rust based Multi-Modal Hyper Caption Tool

## I. Usage

### 1. For a single image/video, it will returns a `*.txt`.
```bash
rimc -f ${file_path} -c `config.toml`
```

### 2. For a batch of image/video list, it will returns a list of `*.txt`.

```bash
rimc -d ${dir_path} -c `config.toml`
```

## II. Example of `config.toml`

```bash

```