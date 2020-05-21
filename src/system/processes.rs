extern crate psutil;

extern crate systemstat;

extern crate log;
extern crate simple_logger;

use std::thread;
use std::time::Duration;

use psutil::process::processes;
use psutil::process::Process;

use psutil::*;

use systemstat::{System, Platform, saturating_sub_bytes};

use crate::system::constants;

use std::process::Command;

pub struct disk_hw {
    pub _percentage_usage: f32,
    pub _memory_left: u64,
    pub _temperature: f32,
}

pub struct Processes {
    _status: bool,
}

impl Processes {
    pub fn new() -> Processes {
        Processes { _status: false }
    }

    pub fn ps_find_pid(&mut self, pid: u32) -> bool {
        let mut found: bool = false;
        let mut processes_vector = processes();
        match processes_vector {
            Ok(output) => trace!("No issue in processes: {:?}", output),
            Err(e) => return found,
        };
        let mut processes = processes().unwrap();
        let block_time = Duration::from_millis(1000);
        thread::sleep(block_time);

        for p in processes {
            if p.is_err() {
                error!("Crash on p");
                return found;
            }
            let p = p.unwrap();
            trace!("Creating check of pid");
            if p.cmdline().is_err() {
                error!("Cmdline failed");
                return found;
            }
            if p.pid() == pid {
                found = true;
            }
        }
        return found;
    }

    pub fn ps_find(&mut self, component: &str) -> u16 {
        let mut amount_found: u16 = 0;
        let mut processes_vector = processes();
        match processes_vector {
            Ok(output) => trace!("No issue in processes: {:?}", output),
            Err(e) => return amount_found,
        };
        let mut processes = processes().unwrap();
        let block_time = Duration::from_millis(1000);
        thread::sleep(block_time);

        for p in processes {
            if p.is_err() {
                error!("Process failed to unwrap");
                return amount_found;
            }
            let p = p.unwrap();
            trace!("Creating check of process");
            if p.cmdline().is_err() {
                error!("Cmdline failed");
                return constants::ERROR_FIND;
            }
            let process = p
                .cmdline()
                .unwrap()
                .unwrap_or_else(|| format!("[{}]", p.name().unwrap()));
            if process.contains(component) {
                amount_found += 1;
            }
        }
        debug!("Amount of processes: {}", amount_found);
        return amount_found;
    }

    pub fn get_disk_usage(&mut self) -> disk_hw{
        let disk_usage = disk::disk_usage("/").unwrap();
        debug!("Disk usage: {:?}", disk_usage);
        let percentage = disk_usage.percent();
        debug!("Disk usage %: {:?}", percentage);
        //
        let sys = System::new();
        //
        let mut temperatures = 0.0;
        match sys.cpu_temp() {
            Ok(cpu_temp) => temperatures = cpu_temp,
            Err(x) => error!("CPU temp: {}", x)
        }   
        //
        let mut memory_left = 0;
        match sys.memory() {
            Ok(mem) => memory_left = mem.free.as_u64(),
            Err(x) => println!("\nMemory: error: {}", x)
        }
        let temp = disk_hw {
            _percentage_usage: percentage,
            _memory_left: memory_left,
            _temperature: temperatures,
        };
        return temp;
    }
}
