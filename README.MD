<!--
 * @Author: Image image@by.cx
 * @Date: 2022-12-05 22:39:47
 * @LastEditors: Image image@by.cx
 * @LastEditTime: 2022-12-07 22:20:16
 * @FilePath: /lookbusy-rs/README.MD
 * @Description: 
 * 
 * Copyright (c) 2022 by Image image@by.cx, All Rights Reserved. 
-->
# Lookbusy - make your PC busy

[crates.io](https://crates.io/crates/lookbusy)![Crates.io](https://img.shields.io/crates/l/lookbusy)![Crates.io](https://img.shields.io/crates/v/lookbusy)![Crates.io](https://img.shields.io/crates/d/lookbusy)

This is a program can be set to eat you cpu and memory resource.

# feature
 - precise control cpu core usage 0/1,3/1,5/1 core0,3,5 use 100% cpu
 - fake log print
 - cpu usage per core

![screenshot](example/example.gif)

# Platform
- Windows
  - x86 & x64 [OK]
- Linux
    - x86 & x64 [OK]
    - arm64 [OK]
- MacOS
  - x86 & x64 [OK]
  - M serial [core control not work]

# Install
```
cargo install lookbusy
```
# How to use
```shell
USAGE:
    lookbusy [OPTIONS]

OPTIONS:
    -c, --cpu-num <CPU_NUM>      how many cpu thread you want use [default: 1]
    -C, --config <CONFIG>        precise control cpu core usage 0/1,3/1,5/1 core0,3,5 use 100% cpu
    -h, --help                   Print help information
    -l, --limit <LIMIT>          cpu usage per thread % [default: 1]
    -L, --log-path <LOG_PATH>    fake log print
    -m, --mem-size <MEM_SIZE>    how many MB you want use [default: 1024]
    -V, --version                Print version information

```

Just like `./lookbusy -c 2 -m 4096`