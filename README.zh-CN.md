Language : [🇺🇸 English](./README.md) | 🇨🇳 简体中文

<h1 align="center">port-killer</h1>
<div align="center">

[![Build Status](https://img.shields.io/crates/v/port-killer)](https://crates.io/crates/port-killer)
![Crates Downloads](https://img.shields.io/crates/d/port-killer)
![Last Commit](https://img.shields.io/github/last-commit/ZingerLittleBee/port-killer-rs)

</div>
<div align="center">

[![Docs](https://img.shields.io/docsrs/port-killer)](https://docs.rs/port-killer/0.1.0/port_killer/)
[![GitHub Actions CI](https://img.shields.io/github/workflow/status/ZingerLittleBee/port-killer-rs/Test%20CI)](https://github.com/ZingerLittleBee/port-killer-rs/actions)
[![LICENSE](https://img.shields.io/crates/l/port-killer)](./LICENSE)

</div>

## Overview
port-killer 是一个 rust 的库, 提供**清除端口占用**、**杀死进程**的功能.

## Installation
1. 获取最新版本 -> https://crates.io/crates/port-killer

2. 添加依赖
```toml
[dependencies]
port-killer = "0.1.0"
```

3. 使用
```rust
use port_killer::{kill, kill_by_pids};

fn main() {
    assert!(kill(5000).expect(""));
    assert!(kill_by_pids(&[56812]).expect(""));
}
```

## Goods
fn -> [kill](#kill) · [kill_by_pids](#kill_by_pids)

## Documentation
### `kill`
根据 port 清除端口占用
```rust
pub fn kill(port: u16) -> Result<bool, Error>
```

### `kill_by_pids`
根据 pids 杀死进程
```rust
pub fn kill_by_pids(pids: &[u32]) -> Result<bool, Error>
```
