extern crate psutil;

extern crate log;
extern crate simple_logger;

use std::thread;
use std::time::Duration;

use psutil::process::processes;
use psutil::process::Process;

use psutil::*;

use crate::system::constants;

use std::process::Command;

pub struct disk_hw {
    pub _percentage_usage: f32,
    pub _uptime: u64,
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
        let uptime = host::uptime().unwrap().as_secs();
        debug!("System uptime: {:?}", uptime);
        let temperatures = sensors::temperatures();
        debug!("System temperature: {:?}", temperatures);
        let temp = disk_hw {
            _percentage_usage: percentage,
            _uptime: uptime,
        };
        return temp;
    }
}
