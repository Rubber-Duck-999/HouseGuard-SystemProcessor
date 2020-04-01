extern crate psutil;

extern crate log;
extern crate simple_logger;

use std::thread;
use std::time::Duration;

use psutil::process::processes;
use psutil::process::Process;

use crate::system::constants;

use std::process::Command;

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

    pub fn start_process(&mut self, component: &str) {
        debug!("Starting process : {}", component);
        let status = Command::new("sh").arg(component).spawn();
        trace!("Status of run: {:?}", status);
    }

    pub fn kill_component(&mut self, component: &str, restart: bool) -> bool {
        let found = self.ps_find(component);
        let mut success: bool = false;
        let result = match found {
            0 => {
                debug!("No process found");
                if restart {
                    self.start_process(component);
                }
            }
            1 => {
                debug!("Component found once");
                success = self.kill_main_component(component);
            }
            _ => {
                success = self.kill_duplicate_component(component);
            }
        };
        return success;
    }

    pub fn kill_main_component(&mut self, component: &str) -> bool {
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

    pub fn kill_duplicate_component(&mut self, component: &str) -> bool {
        let mut found: i32 = 0;
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
                if found > 0 {
                    success = self.kill_component_pid(p.pid());
                }
                found += 1;
            }
        }
        return success;
    }

    pub fn kill_component_pid(&mut self, component: u32) -> bool {
        warn!("Killing {}", component);
        let mut error_present: bool = false;
        let process = Process::new(component).unwrap();
        if self.ps_find_pid(process.pid()) {
            if let Err(error) = process.kill() {
                error!("Failed to kill process: {}.", error);
                error_present = true;
            }
        } else {
            error_present = true;
        }
        return error_present;
    }
}
