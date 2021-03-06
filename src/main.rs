mod rabbitmq;
mod system;

use clap::App;

use std::process;

extern crate chrono;
use chrono::prelude::*;

use std::{thread, time};

#[macro_use]
extern crate log;
extern crate simple_logger;

use log::Level;

use std::fs;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate system_shutdown;

extern crate glob;

struct Control {
    _process: system::processes::Processes,
    _channel: rabbitmq::interaction::SessionRabbitmq,
    _shutdown: bool,
    _user_panel: bool,
    _fault_handler: bool,
    _camera_monitor: bool,
    _network_access_controller: bool,
    _sql: bool,
    _rabbitmq: bool,
    _highest_disk_usage: u32,
    _temperature: u32,
    _memory_left: u64,
}

impl Control {
    pub fn new() -> Control {
        Control {
            _process: system::processes::Processes::new(),
            _channel: rabbitmq::interaction::SessionRabbitmq {
                ..Default::default()
            },
            _shutdown: false,
            _user_panel: true,
            _fault_handler: true,
            _camera_monitor: true,
            _network_access_controller: true,
            _sql: true,
            _rabbitmq: true,
            _highest_disk_usage: 0,
            _temperature: 0,
            _memory_left: 0,
        }
    }

    fn get_status_update(&mut self) {
        let disk:system::processes::DiskHw = self._process.get_disk_usage();
        let mut updated = false;
        debug!("Temperature {}", disk._temperature);
        debug!("Temperature {}", self._temperature);
        if self._temperature != disk._temperature {
            self._temperature = disk._temperature as u32;
            updated = true;
        }
        if self._memory_left < disk._memory_left {
            self._memory_left = disk._memory_left;
            updated = true;
        }
        debug!("Current disk usage {}", disk._percentage_usage);
        if disk._percentage_usage > self._highest_disk_usage {
            warn!("Setting new disk usage");
            self._highest_disk_usage = disk._percentage_usage as u32;
        }
        if updated {
            let status = rabbitmq::types::StatusSYP {
                temperature: self._temperature,
                memory_left: self._memory_left,
                highest_usage: self._highest_disk_usage,
            };
            let serialized = serde_json::to_string(&status).unwrap();
            debug!("Message: {}", serialized);
            self._channel.publish(rabbitmq::types::STATUS_SYP, &serialized);
        }
    }

    fn get_time(&mut self) -> String {
        let dt = Utc::now();
        return dt.format("%Y/%m/%d %H:%M:%S").to_string();
    }

    fn publish_failure_component(&mut self, component: &str) {
        if self._rabbitmq == true {
            let failure = rabbitmq::types::FailureComponent {
                time: self.get_time(),
                type_of_failure: component.to_string(),
            };
            let serialized = serde_json::to_string(&failure).unwrap();
            self._channel.publish(rabbitmq::types::FAILURE_COMPONENT, &serialized);
        } else {
            error!("Rabbitmq has not been running, cannot send message");
        }
    }

    fn check_file(&mut self, file: &str) -> bool {
        debug!("Checking file exists {}", file);
        let _b = std::path::Path::new(file).exists();
        return _b;
    }

    fn compare(&mut self, file: &str, text: &str) -> bool {
        debug!("Comparing file");
        let data = fs::read_to_string(file).expect("Unable to read file");
        if data.contains(text) {
            return true;
        } else {
            return false;
        }
    }

    fn check_user_panel(&mut self) {
        let component = "UserPanel";
        debug!("Looking for {}", component);
        let mut found = self._process.ps_find(&component); 
        while found == system::constants::ERROR_FIND {
            debug!("Find failed to run, retrying");
            found = self._process.ps_find(&component);
        }
        if found == 0 && self._user_panel == true {
            error!("The component was not alive please debug {}", component);
            self.publish_failure_component(system::constants::USER_PANEL);
            self._user_panel = false;
        } else if found == 0 && self._user_panel != true {
            debug!("The component is still dead {}", component);
        } else if found >= 1 && self._user_panel == false {
            warn!("The component is now alive {}", component);
            self._user_panel = true;
        } else {
            self._user_panel = true;
        }
    }

    fn check_camera_monitor(&mut self) {
        let component = "exeCameraMonitor";
        debug!("Looking for {}", component);
        let mut found = self._process.ps_find(&component); 
        while found == system::constants::ERROR_FIND {
            debug!("Find failed to run, retrying");
            found = self._process.ps_find(&component);
        }
        if found == 0 && self._camera_monitor == true {
            error!("The component was not alive please debug {}", component);
            self.publish_failure_component(system::constants::CAMERA_MONITOR);
            self._camera_monitor = false;
        } else if found == 0 && self._camera_monitor != true {
            debug!("The component is still dead {}", component);
        } else if found >= 1 && self._camera_monitor == false {
            warn!("The component is now alive {}", component);
            self._camera_monitor = true;
        } else {
            self._camera_monitor = true;
        }      
    }

    fn check_fault_handler(&mut self) {
        let component = "exeFaultHandler";
        debug!("Looking for {}", component);
        let mut found = self._process.ps_find(&component); 
        while found == system::constants::ERROR_FIND {
            debug!("Find failed to run, retrying");
            found = self._process.ps_find(&component);
        }
        if found == 0 {
            debug!("Rerunning find to ensure its definitely not a failure");
            if self._process.ps_find(&component) == 0
            {
                if self._fault_handler == true {
                    self._fault_handler = false;
                    error!("Fault handler is not alive, we should restart the system");
                    /*
                    match system_shutdown::reboot() {
                        Ok(_) => println!("Rebooting ..."),
                        Err(error) => eprintln!("Failed to reboot: {}", error),
                    }*/
                } else if self._fault_handler == false {
                    debug!("Fault Handler is still dead");
                }
            }
        }
    }

    fn check_network_access_controller(&mut self) {
        let component = "exeNetworkAccessController";
        debug!("Looking for {}", component);
        let mut found = self._process.ps_find(&component); 
        while found == system::constants::ERROR_FIND {
            debug!("Find failed to run, retrying");
            found = self._process.ps_find(&component);
        }
        if found == 0 && self._network_access_controller == true {
            error!("The component was not alive {}", component);
            self.publish_failure_component(system::constants::NETWORK_ACCESS_CONTROLLER);
            self._network_access_controller = false;
        } else if found == 0 && self._network_access_controller != true {
            debug!("The component is still dead {}", component);
        } else if found >= 1 && self._network_access_controller == false {
            warn!("The component is now alive {}", component);
            self._network_access_controller = true;
        } else {
            self._network_access_controller = true;
        }
    }

    fn check_sql(&mut self) {
        let component = "mysql";
        debug!("Looking for {}", component);
        let mut found = self._process.ps_find(&component);
        while found == system::constants::ERROR_FIND {
            debug!("Find failed to run, retrying");
            found = self._process.ps_find(&component);
        }
        if found == 0 && self._sql == true {
            error!("The component was not alive {}", component);
            self.publish_failure_component(system::constants::SQL);
            self._sql = false;
        } else if found == 0 && self._sql != true {
            debug!("The component is still dead {}", component);
        } else if found >= 1 && self._sql == false {
            warn!("The component is now alive {}", component);
            self._sql = true;
        } else {
            debug!("The component is alive {}", component);
            self._sql = true;
        }
    }

    fn check_rabbitmq(&mut self) {
        let component = "rabbitmq";
        debug!("Looking for {}", component);
        let mut found = self._process.ps_find(&component); 
        while found == system::constants::ERROR_FIND {
            debug!("Find failed to run, retrying");
            found = self._process.ps_find(&component);
        }
        if found == 0 && self._rabbitmq == true {
            error!("The component was not alive {}", component);
            self.publish_failure_component(system::constants::SQL);
            self._rabbitmq = false;
        } else if found == 0 && self._rabbitmq != true {
            debug!("The component is still dead {}", component);
        } else if found >= 1 && self._rabbitmq == false {
            warn!("The component is now alive {}", component);
            self._rabbitmq = true;
        } else {
            debug!("The component is alive {}", component);
            self._rabbitmq = true;
        }
    }

    fn check_messages(&mut self) {
        let x:rabbitmq::interaction::MessagePower = self._channel.consume_get();
    }

    pub fn control_loop(&mut self) {
        trace!("Declaring consumer...");
        self._channel.consume();
        thread::sleep(time::Duration::from_secs(60));
        while self._shutdown != true {
            self.check_messages();
            self.check_rabbitmq();
            self.check_fault_handler();
            self.check_sql();
            self.check_network_access_controller();
            self.check_camera_monitor();
            self.check_user_panel();
            self.get_status_update();
            thread::sleep(time::Duration::from_secs(60));
        }
    }
}

fn main() {
    simple_logger::init_with_level(Level::Debug).unwrap();

    if log_enabled!(Level::Trace) {
        info!("Logging has been enabled to trace");
    }

    App::new("exeSystemProcessor")
        .version("1.3.0")
        .about("The hearbeat and monitor for HouseGuard.");

    let mut control = Control::new();

    control.control_loop();
    process::exit(0);
}
