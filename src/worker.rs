use std::mem::size_of;
use std::thread;
use std::time::{Duration, SystemTime};
use rand::Rng;
use crate::configure::CPU;
use crate::{EAT_MEM, HANDLES};

pub struct CPUWorker{

}
impl CPUWorker{
    pub fn new()->CPUWorker{
        CPUWorker{

        }
    }
    pub fn stress_accurate(self, config:Vec<CPU>){
        let core_ids = core_affinity::get_core_ids().unwrap();
        for i in config{
            let core = core_ids[i.id as usize];
            let handle = thread::spawn(move || {
                let mut rng = rand::thread_rng();
                let res = core_affinity::set_for_current(core);
                println!("Set core {} result: {}", i.id, res);
                loop{
                    let durations = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
                    let limit = i.limit.unwrap_or(1.0);
                    let jitter = i.jitter.unwrap_or(0.0);
                    if durations.as_millis() % 1000 > (limit * 1000.0) as u128 {
                        thread::sleep(Duration::from_millis(20 + (jitter * rng.gen::<f32>() * 1000.0) as u64));
                    }
                    _ = 2 * 11;
                };
            });
            unsafe{
                HANDLES.push(handle);
            };
        }
    }
    pub fn stress(self, cpu_num:usize, limit:f32) {
        for _i in 0..cpu_num {
            let handle = thread::spawn(move || {
                loop {
                    let durations = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
                    if durations.as_millis() % 1000 > (limit * 1000.0) as u128 {
                        thread::sleep(Duration::from_millis(10));
                    }
                    let _answer = 2 * 21;
                };
            });
            unsafe {
                HANDLES.push(handle);
            };
        }
    }
}
pub struct MemWorker{

}
impl MemWorker{
    pub fn new()->MemWorker{
        MemWorker{

        }
    }
    pub fn busy(self, size_mb:u64){
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
}