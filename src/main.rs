/*
 * @Author: Image image@by.cx
 * @Date: 2022-12-05 21:40:45
 * @LastEditors: Image_woker_pc image@by.cx
 * @LastEditTime: 2022-12-07 10:50:01
 * @FilePath: /lookbusy-rs/src/main.rs
 * @Description: 
 * 
 * Copyright (c) 2022 by Image image@by.cx, All Rights Reserved. 
 */
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime};
use ctrlc;
use clap::Parser;
use rand::Rng;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args{
    #[clap(short, long, default_value_t=1)]
    cpu_num: u64,
    #[clap(short, long, default_value_t=1024)]
    mem_size: u64,
}
static mut HANDLES:Vec<JoinHandle<bool>> = vec![];
static mut EAT_MEM:Vec<u64> = vec![];
fn cpu_busy(cpu_num:u64){
    for _i in 0..cpu_num{
        let handle = thread::spawn(|| {
            loop{
                let durations = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
                if durations.as_millis() % 1000 > 500{
                    thread::sleep(Duration::from_millis(500));
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
    for _i in 0..target_size_bit{
        unsafe{
            EAT_MEM.push(rand::thread_rng().gen());
        }
        
    }
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
    cpu_busy(args.cpu_num);
    unsafe{
        for i in HANDLES.pop(){
            i.join().unwrap();
        }
    }
}
