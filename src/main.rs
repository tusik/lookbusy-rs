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
use std::io::{self, BufRead, Write};
use std::fs::File;
use std::mem::size_of;
use std::path::Path;
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime};
use ctrlc;
use clap::Parser;
use rand_distr::{Distribution,SkewNormal};
use sysinfo::{System, SystemExt};
use rand::thread_rng;
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
    /// precise control cpu core usage
    /// 0/1,3/1,5/1 core0,3,5 use 100% cpu
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
            let disr = SkewNormal::new(20.0, 300.0, 0.5).unwrap();
            let mut rng = thread_rng();
            for line in lines {
                if let Ok(ip) = line {
                    let num = disr.sample(&mut rng)as u64;
                    println!("{}", ip);
                    thread::sleep(Duration::from_millis(num));
                }      
            }   
        }
    }
}
fn cpu_busy(cpu_num:usize, limit:f32){
    for _i in 0..cpu_num{
        let handle = thread::spawn(move || {
            loop{
                let durations = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
                if durations.as_millis() % 1000 > (limit * 1000.0) as u128{
                    thread::sleep(Duration::from_millis(10));
                }
                let _answer = 2 * 21;

            };
        });
        unsafe{
            HANDLES.push(handle);
        };
    }
}

fn cpu_busy_accurate(config:Vec<(u32,f32)>){
    let core_ids = core_affinity::get_core_ids().unwrap();
    for i in config{
        let core = core_ids[i.0 as usize];
        let handle = thread::spawn(move || {
            core_affinity::set_for_current(core);
            loop{
                let durations = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
                if durations.as_millis() % 1000 > (i.1 * 1000.0) as u128{
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
    println!("Start eat memory!!!");
    let data_size = size_of::<u128>() as u64;
    let target_size_bit = size_mb *1024 *1024 *8 / (data_size * 8);
    let start_durations = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    unsafe{
        EAT_MEM.resize(target_size_bit.try_into().unwrap(), 1);
    }
    let end_durations = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let diff = end_durations - start_durations;
    println!("Eat memory takes {} ms.",diff.as_millis());
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
                            std::io::stdout().flush().expect("Error on message flush.");
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
    mem_busy(args.mem_size);
    let mut cpus = Vec::new();
    if args.config.is_some(){
        let config = args.config.unwrap();
        let parts:Vec<&str> = config.split(",").collect();
        for part in parts{
            let _part = part.trim();
            let _part:Vec<&str> = _part.split("/").collect();
            let core = _part[0].parse::<u32>().unwrap();
            let limit = _part[1].parse::<f32>().unwrap();
            cpus.push((core,limit));
        }
    }
    if cpus.len() > 0{
        cpu_busy_accurate(cpus.clone());
        print!("Now I'm eat {:} cpu", &cpus.len());
    }else{
        cpu_busy(args.cpu_num,args.limit);
        print!("Now I'm eat {:} cpu", args.cpu_num);
    }
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
