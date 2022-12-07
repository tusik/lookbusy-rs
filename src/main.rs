/*
 * @Author: Image image@by.cx
 * @Date: 2022-12-05 21:40:45
 * @LastEditors: Image image@by.cx
 * @LastEditTime: 2022-12-07 22:38:36
 * @FilePath: /lookbusy-rs/src/main.rs
 * @Description: 
 * 
 * Copyright (c) 2022 by Image image@by.cx, All Rights Reserved. 
 */
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime};
use ctrlc;
use clap::Parser;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args{
    /// how many cpu thread you want use.
    #[clap(short, long, default_value_t=1)]
    cpu_num: u64,
    /// cpu usage per thread %.
    #[clap(short, long, default_value_t=1.0)]
    limit: f32,
    /// how many MB you want use.
    #[clap(short, long, default_value_t=1024)]
    mem_size: u64,
}
static mut HANDLES:Vec<JoinHandle<bool>> = vec![];
static mut EAT_MEM:Vec<u64> = vec![];
fn cpu_busy(cpu_num:u64, limit:f32){
    for _i in 0..cpu_num{
        let handle = thread::spawn(move || {
            loop{
                let durations = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
                if durations.as_millis() % 1000 > (limit * 1000.0) as u128{
                    thread::sleep(Duration::from_millis(10));
                }
                _ = 2 * 11;
            };
        });
        unsafe{
            HANDLES.push(handle);
        };
    }
}
fn mem_busy(size_mb:u64){
    let target_size_bit = size_mb *1024 *1024 *8 /64;
    let start_durations = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    unsafe{
        EAT_MEM.resize(target_size_bit.try_into().unwrap(), 1);
    }
    let end_durations = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let diff = end_durations - start_durations;
    println!("{}",diff.as_millis());
}
fn print_info(args:&Args){
    println!("Process start.");
    println!("Now I'm eat {:} cpu and {:} MB Memory.",
        args.cpu_num, args.mem_size);
    println!("Use Ctrl + C to stop.");
}
fn main() {
    let args = Args::parse();
    print_info(&args);
    ctrlc::set_handler(||{
        println!("\nTask Finished! bye~");
        std::process::exit(0);        
    }).expect("Error setting Ctrl-C handler");
    mem_busy(args.mem_size);
    cpu_busy(args.cpu_num,args.limit);
    unsafe{
        for i in HANDLES.pop(){
            i.join().unwrap();
        }
    }
}
