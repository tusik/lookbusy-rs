/*
 * @Author: Image image@by.cx
 * @Date: 2022-12-05 21:40:45
 * @LastEditors: Image image@by.cx
 * @LastEditTime: 2023-03-22 15:56:07
 * @FilePath: /lookbusy-rs/src/main.rs
 * @Description: 
 * 
 * Copyright (c) 2022 by Image image@by.cx, All Rights Reserved. 
 */
mod configure;
mod worker;

use std::io::{self, BufRead, Write};
use std::fs::File;
use std::path::Path;
use std::thread::{self, JoinHandle};
use std::time::{Duration};
use ctrlc;
use clap::Parser;
use sysinfo::{System, SystemExt};
use rand::Rng;
use crate::configure::{Config};
use crate::worker::{CPUWorker, MemWorker};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args{
    /// how many cpu thread you want use.
    #[clap(short, long, env, default_value_t=1)]
    cpu_num: usize,
    /// cpu usage per thread %.
    #[clap(short, long, env, default_value_t=1.0)]
    limit: f32,
    /// how many MB you want use.
    #[clap(short, long, env, default_value_t=1024)]
    mem_size: u64,
    /// fake log print.
    #[clap(short='L', long, env)]
    log_path: Option<String>,
    /// precise control cpu core by config file
    /// -C config.toml
    #[clap(short='C', long, env)]
    config: Option<String>
}
static mut HANDLES:Vec<JoinHandle<bool>> = vec![];
static mut EAT_MEM:Vec<u128> = vec![];
static STICK: [char; 3] = ['_','-','_'];
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn display_log(log_path:&str){
    loop {
        print!("\x1b[2J");
        print!("\x1b[H");
        if let Ok(lines) = read_lines(log_path) {
            let mut rng = rand::thread_rng();
            for line in lines {
                if let Ok(ip) = line {
                    println!("{}", ip);
                    thread::sleep(Duration::from_millis(rng.gen_range(2..200)));
                }      
            }   
        }
    }
}

fn print_info(){
    println!("Process start.");
    println!("Use Ctrl + C to stop.");
}

fn processing_display(log_path : Option<String>) -> JoinHandle<bool>{
    thread::spawn(move||{
        let mut stick_itr = STICK.iter();
        match log_path {
            Some(path) => display_log(&path),
            None => {
                loop{
                    match stick_itr.next() {
                        Some(chr) => {
                            print!("Running {} \r",chr);
                            io::stdout().flush().expect("Error on message flush.");
                        },
                        None => {
                            stick_itr = STICK.iter();
                        },
                    }
                    thread::sleep(Duration::from_millis(100));
                }
            },
        }
        return true;
    })
}

fn main() {
    let sys = System::new_all();
    let args = Args::parse();
    print_info();
    ctrlc::set_handler(||{
        println!("\nTask Finished! bye~");
        std::process::exit(0);        
    }).expect("Error setting Ctrl-C handler");
    let free_mem = sys.total_memory() - sys.used_memory();
    if free_mem <= args.mem_size * 1024 * 1024 {
        println!("\nWarning! Free memory is less than require. It could cause system performance issue. ");
    }
    println!("Initializing CPU/Mem worker");
    let cpu_worker = CPUWorker::new();
    if args.config.is_some(){
        let config = match Config::load(args.config.unwrap()){
            Ok(c) => {c}
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };
        cpu_worker.stress_accurate(config.cpu.unwrap());
    }else{
        cpu_worker.stress(args.cpu_num,args.limit);
    }
    let mem_worker = MemWorker::new();
    mem_worker.busy(args.mem_size);

    println!(" and {:} MB Memory.", args.mem_size);
    let processing_handle = processing_display(args.log_path);
    unsafe{
        HANDLES.push(processing_handle);
    };
    unsafe{
        while let Some(handle) = HANDLES.pop() {
            let _ = handle.join().unwrap();
        }
    }
}
