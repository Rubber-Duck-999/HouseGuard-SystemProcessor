extern crate psutil;

extern crate log;
extern crate simple_logger;

use std::thread;
use std::time::Duration;

use psutil::process::Process;
use psutil::process::processes;

use std::process::Command;

pub struct Processes {
    _status : bool,
}

impl Processes {
    pub fn new() -> Processes {
        Processes {
            _status: false,
        }
    }

    pub fn ps_find_pid(&mut self, pid: i32) -> bool {
        let mut found: bool = false;
        /*
        for p in &psutil::process::all().unwrap() {
            if p.pid == pid {
                found = true;
            }
        }*/
        return found;
    }

    pub fn ps_find(&mut self, component: &str) -> u16 {
        let mut amount_found: u16 = 0;
        warn!("Running");
        let processes = processes().unwrap();
        let block_time = Duration::from_millis(1000);
        thread::sleep(block_time);
        
        for p in processes {
            let mut p = p.unwrap();
            debug!(
                "{:.100}",
                p.cmdline()
                    .unwrap()
                    .unwrap_or_else(|| format!("[{}]", p.name().unwrap())),
            );
            let name = match p.name() {
                Ok(output) => warn!("{:?}", output),
                Err(e) => warn!("Error parsing {:?}", e),
            };
            /*
            if component.contains(p.name()) {
                amount_found += 1;
            }*/
        }
        warn!("Amount of processes: {}", amount_found);
        return amount_found;
    }

    pub fn start_process(&mut self, component: &str) {
        warn!("Starting process : {}", component);
        let status = Command::new("sh").arg(component).spawn();
        warn!("Status of run: {:?}", status);
        
    }

    pub fn kill_component(&mut self, component: &str, restart: bool) {
        let found = self.ps_find(component);
        let result = match found {
            0 => {
                warn!("No process found");
                if restart {
                    self.start_process(component);
                }
            }
            1 => {
                warn!("Component found once");
                self.kill_main_component(component);
            }
            _ => {
                self.kill_duplicate_component(component);
            }
        };
    }

    pub fn kill_main_component(&mut self, component: &str) -> bool {
        let mut success: bool = false;
        /*
        for p in &psutil::process::all().unwrap() {
            let mut cmd = p
                .cmdline()
                .unwrap()
                .unwrap_or_else(|| format!("[{}]", p.comm));
            if cmd.contains(component) {
                warn!("Found Process : Key {}, {}", cmd, component);
                if !self.kill_component_pid(p.pid) {
                    success = true;
                }
            }
        }*/
        return success;
    }

    pub fn kill_duplicate_component(&mut self, component: &str) {
        let mut found: i32 = 0;
        /*
        for p in &psutil::process::all().unwrap() {
            let mut cmd = p
                .cmdline()
                .unwrap()
                .unwrap_or_else(|| format!("[{}]", p.comm));
            if cmd.contains(component) {
                warn!("Found Process : Key {}, {}", cmd, component);
                if found > 0 {
                    if !self.kill_component_pid(p.pid) {}
                }
                found += 1;
            }
        }*/
    }

    pub fn kill_component_pid(&mut self, component: i32) -> bool {
        let mut error_present: bool = false;
        /*
        let process = Process::new(component).unwrap();
        if self.ps_find_pid(process.pid) {
            if let Err(error) = process.kill() {
                error!("Failed to kill process: {}.", error);
                error_present = true;
            }
        } else {
            error_present = true;
        }*/
        return error_present;
    }
}
