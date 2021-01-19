extern crate psutil;

extern crate systemstat;

extern crate log;
extern crate simple_logger;

use std::thread;
use std::time::Duration;

use psutil::process::processes;
use psutil::process::Process;

use psutil::*;

use systemstat::{System, Platform};

use crate::system::constants;

use std::process::Command;

pub struct DiskHw {
    pub _percentage_usage: u32,
    pub _memory_left: u64,
    pub _temperature: u32,
}

pub struct Processes {
    _status: bool,
}

impl Processes {
    pub fn new() -> Processes {
        Processes { _status: false }
    }

    pub fn start_cm(&mut self) {
        warn!("Starting process : CameraMonitor");
        let status = Command::new("python3").arg("exeCameraMonitor.py -c conf.json").spawn();
        warn!("Status of run: {:?}", status);
    }

    pub fn kill_cm(&mut self, component: &str) -> bool {
        let mut success: bool = false;
        let mut processes_vector = processes();
        match processes_vector {
            Ok(output) => trace!("No issue in processes: {:?}", output),
            Err(e) => return success,
        };
        let mut processes = processes().unwrap();
        let block_time = Duration::from_millis(1000);
        thread::sleep(block_time);

        for p in processes {
            if p.is_err() {
                error!("How?");
                return success;
            }
            let p = p.unwrap();
            trace!("Creating check of process");
            if p.cmdline().is_err() {
                error!("Cmdline failed");
                return success;
            }
            let process = p
                .cmdline()
                .unwrap()
                .unwrap_or_else(|| format!("[{}]", p.name().unwrap()));

            if process.contains(component) {
                debug!("Found Process : Key {}", component);
                if !self.kill_component_pid(p.pid()) {
                    success = true;
                }
            }
        }
        return success;
    }

    pub fn kill_component_pid(&mut self, component: u32) -> bool {
        warn!("Killing {}", component);
        let mut error_present: bool = false;
        let process = Process::new(component).unwrap();
        if let Err(error) = process.kill() {
            error!("Failed to kill process: {}.", error);
            error_present = true;
        }
        return error_present;
    }

    pub fn ps_find(&mut self, component: &str) -> u16 {
        let mut amount_found: u16 = 0;
        let processes_vector = processes();
        match processes_vector {
            Ok(output) => trace!("No issue in processes: {:?}", output),
            Err(_e) => return amount_found,
        };
        let processes = processes().unwrap();
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

    pub fn get_disk_usage(&mut self) -> DiskHw {
        let disk_usage = disk::disk_usage("/").unwrap();
        debug!("Disk usage: {:?}", disk_usage);
        let percentage = disk_usage.percent() as u32;
        debug!("Disk usage %: {:?}", percentage);
        //
        let sys = System::new();
        //
        let mut temperature = 0;
        match sys.cpu_temp() {
            Ok(cpu_temp) => temperature = cpu_temp as u32,
            Err(x) => error!("CPU temp: {}", x)
        }   
        //
        let mut memory_left = 0;
        match sys.memory() {
            Ok(mem) => memory_left = mem.free.as_u64(),
            Err(x) => println!("\nMemory: error: {}", x)
        }
        for i in 1..3 {
            memory_left = memory_left / 1024;
        }
        let temp = DiskHw {
            _percentage_usage: percentage,
            _memory_left: memory_left,
            _temperature: temperature,
        };
        return temp;
    }
}
