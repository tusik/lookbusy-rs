use std::thread::{self, JoinHandle};
use clap::Parser;
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
    let c = 14;
    for _i in 0..cpu_num{
        let handle = thread::spawn(move ||{
            loop{
                _ = c*11;
            };
        });
        unsafe{
            HANDLES.push(handle);
        };
    }
}
fn mem_busy(size_mb:u64){
    let target_size_bit = size_mb *1024 *1024 *8 /64;
    for _i in 0.. target_size_bit{
        unsafe{
            EAT_MEM.push(1);
        }
        
    }
}
fn main() {
    let args = Args::parse();
    mem_busy(args.mem_size);
    cpu_busy(args.cpu_num);
    unsafe{
        for i in HANDLES.pop(){
            i.join().unwrap();
        }
    }
}
