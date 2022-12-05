<!--
 * @Author: Image image@by.cx
 * @Date: 2022-12-05 22:39:47
 * @LastEditors: Image image@by.cx
 * @LastEditTime: 2022-12-05 22:47:35
 * @FilePath: /lookbusy-rs/src/README.MD
 * @Description: 
 * 
 * Copyright (c) 2022 by Image image@by.cx, All Rights Reserved. 
-->
# Lookbusy - make your PC looks busy

This is a program can be set to eat you cpu and memory resource.

Keep at least one cpu thread free. It's good for you. 

# How to use
```shell
USAGE:
    lookbusy [OPTIONS]

OPTIONS:
    -c, --cpu-num <CPU_NUM>      [default: 1]
    -h, --help                   Print help information
    -m, --mem-size <MEM_SIZE>    [default: 1024]
    -V, --version                Print version information
```

Just like `./lookbusy -c 2 -m 4096`