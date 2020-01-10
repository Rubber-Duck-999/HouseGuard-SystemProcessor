extern crate psutil;

extern crate log;
extern crate simple_logger;

use psutil::process::Process;
use std::collections::HashMap;

use std::process::Command;

struct Component 
{
    _name: String,
    _pid: i32,
    _alive: bool,
}

pub struct Processes 
{
    component_map: HashMap<u16, Component>,
    id_key: u16,
}

impl Processes 
{
    pub fn new() -> Processes 
    {
        Processes 
        {
            component_map: HashMap::new(),
            id_key: 0,
        }
    }

    fn join(&mut self, component: Component) 
    {
        warn!("Adding found process to map");
        self.component_map.insert(self.id_key, component); // inserting moves `node`
        self.id_key += 1;
    }

    fn clear_map(&mut self) 
    {
        self.component_map.clear();
    }


    pub fn find(&mut self, component: &str) ->bool
    {
        let mut found:bool = false;

        for p in &psutil::process::all().unwrap() 
        {
            let mut cmd = p
                .cmdline()
                .unwrap()
                .unwrap_or_else(|| format!("[{}]", p.comm));
            if cmd.contains(component) 
            {
                found = true;
            }
        }
        return found;
    }

    pub fn ps_find(&mut self, component: &str) -> u16 
    {
        let mut amount_found: u16 = 0;

        for p in &psutil::process::all().unwrap() 
        {
            let mut cmd = p
                .cmdline()
                .unwrap()
                .unwrap_or_else(|| format!("[{}]", p.comm));
            if cmd.contains(component) 
            {
                trace!("Found program and listing details");
                trace!(
                    "{:>5} {:^5} {:>8.2} {:>8.2} {:.100}",
                    "PID", "STATE", "UTIME", "STIME", "CMD"
                );
                trace!(
                    "{:>5} {:^5} {:>8.2} {:>8.2} {:.100}",
                    p.pid,
                    p.state.to_string(),
                    p.utime,
                    p.stime,
                    p.cmdline()
                        .unwrap()
                        .unwrap_or_else(|| format!("[{}]", p.comm))
                );
                let this_pid = p.pid;
                let this_alive = true;
                let new = component;
                let inputted = Component 
                {
                    _name: String::from(new),
                    _pid: this_pid,
                    _alive: this_alive,
                };
                self.join(inputted);
                amount_found += 1;
            }
        }
        warn!("Amount of processes: {}", amount_found);
        return amount_found;
    }

    pub fn start_process(&mut self, component: &str) 
    {
        debug!("Starting process : {}", component);
        let status = Command::new("sh").arg(component).spawn();
    }

    pub fn kill_component(&mut self, component: &str, restart: bool) -> bool 
    {
        let found = self.ps_find(component);
        let result = match found 
        {
            0 => {
                warn!("No process found");
                if restart 
                {
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
        return true;
    }
    
    pub fn kill_main_component(&mut self, component: &str) 
    {
        let mut component_vec: Vec<i32> = Vec::new();
        let iter: i8 = 0;
        for (key, val) in self.component_map.iter() 
        {
            warn!(
                "key: {} val name: {} val pid: {} val alive: {}",
                key, val._name, val._pid, val._alive
            );
            if val._name.contains(component) 
            {
                warn!("Found Process : {}", component);
                component_vec.push(val._pid);
            }
        }
        for i in component_vec 
        {
            warn!("Killing duplicate : {} pid : {}", component, i);
            if self.kill_component_pid(i) 
            {
                self.clear_map();
            }
        }
    }
    
    pub fn kill_duplicate_component(&mut self, component: &str) 
    {
        let mut component_vec: Vec<i32> = Vec::new();
        let mut iter: i8 = 0;
        for (key, val) in self.component_map.iter() 
        {
            warn!(
                "key: {} val name: {} val pid: {} val alive: {}",
                key, val._name, val._pid, val._alive
            );
            if val._name.contains(component) 
            {
                iter += 1;
                if iter > 1 
                {
                    warn!("Found Process : {}", component);
                    component_vec.push(val._pid);
                }
            }
        }
        for i in component_vec 
        {
            warn!("Killing duplicate : {} pid : {}", component, i);
            if self.kill_component_pid(i) 
            {
                self.clear_map();
            }
        }
    }

    fn kill_component_pid(&mut self, component: i32) -> bool 
    {
        let mut error_present: bool = false;
        let process = Process::new(component).unwrap();

        if let Err(error) = process.kill() 
        {
            println!("Failed to kill process: {}.", error);
            error_present = true;
        }
        return error_present;
    }
}
