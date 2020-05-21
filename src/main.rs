mod rabbitmq;
mod system;

use clap::App;

use std::process;

use std::ffi::{OsString, OsStr};

extern crate chrono;
use chrono::prelude::*;

use std::{thread, time};

#[macro_use]
extern crate log;
extern crate simple_logger;

use log::Level;

use std::fs;
use std::io::BufReader;
use std::io::prelude::*;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate system_shutdown;

use system_shutdown::shutdown;

extern crate glob;

use glob::glob;
use std::error::Error;

use std::env;

struct Control {
    _process: system::processes::Processes,
    _channel: rabbitmq::interaction::SessionRabbitmq,
    _shutdown: bool,
    _event_counter: u32,
    _userPanel: bool,
    _faultHandler: bool,
    _databaseManager: bool,
    _cameraMonitor: bool,
    _environmentManager: bool,
    _networkAccessController: bool,
    _sql: bool,
    _rabbitmq: bool,
    _highest_disk_usage: f32,
    _temperature: f32,
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
            _event_counter: 0,
            _userPanel: true,
            _faultHandler: true,
            _databaseManager: true,
            _cameraMonitor: true,
            _environmentManager: true,
            _networkAccessController: true,
            _sql: true,
            _rabbitmq: true,
            _highest_disk_usage: 0.0,
            _temperature: 0.0,
            _memory_left: 0,
        }
    }

    fn get_status_update(&mut self) {
        let disk:system::processes::disk_hw = self._process.get_disk_usage();
        let mut updated = false;
        if self._temperature != disk._temperature {
            self._temperature = disk._temperature;
            updated = true;
        }
        if self._memory_left != disk._memory_left {
            self._memory_left = disk._memory_left;
            updated = true;
        }
        debug!("Current disk usage {}", disk._percentage_usage);
        if disk._percentage_usage > self._highest_disk_usage {
            warn!("Setting new disk usage");
            self._highest_disk_usage = disk._percentage_usage;
            if updated && self._highest_disk_usage > 95.0 {
                let event = rabbitmq::types::EventSyp {
                    message: "CPU High Usage".to_string(),
                    time: self.get_time(),
                    component: system::constants::COMPONENT_NAME.to_string(),
                };
                self.send_event(&event);
            }
        }
        if updated {
            let status = rabbitmq::types::StatusSYP {
                temperature: self._temperature,
                memory_left: self._memory_left,
                highest_usage: self._highest_disk_usage,
            };
            let serialized = serde_json::to_string(&status).unwrap();
            self._channel.publish(rabbitmq::types::STATUS_SYP, &serialized);
            self._event_counter += 1;
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
            self._event_counter += 1;
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
        if found == 0 && self._userPanel == true {
            error!("The component was not alive please debug {}", component);
            self.publish_failure_component(system::constants::USER_PANEL);
            self._userPanel = false;
        } else if found == 0 && self._userPanel != true {
            debug!("The component is still dead {}", component);
        } else if found >= 1 && self._userPanel == false {
            warn!("The component is now alive {}", component);
            self._userPanel = true;
        } else {
            self._userPanel = true;
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
        if found == 0 && self._cameraMonitor == true {
            error!("The component was not alive please debug {}", component);
            self.publish_failure_component(system::constants::CAMERA_MONITOR);
            self._cameraMonitor = false;
        } else if found == 0 && self._cameraMonitor != true {
            debug!("The component is still dead {}", component);
        } else if found >= 1 && self._cameraMonitor == false {
            warn!("The component is now alive {}", component);
            self._cameraMonitor = true;
        } else {
            self._cameraMonitor = true;
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
            if(self._process.ps_find(&component) == 0)
            {
                if(self._faultHandler == true) {
                    self._faultHandler = false;
                    error!("Fault handler is not alive, we should restart the system");
                    match system_shutdown::reboot() {
                        Ok(_) => println!("Rebooting ..."),
                        Err(error) => eprintln!("Failed to reboot: {}", error),
                    }
                    let event = rabbitmq::types::EventSyp {
                        message: "FH down - rebooting".to_string(),
                        time: self.get_time(),
                        component: system::constants::COMPONENT_NAME.to_string(),
                    };
                    self.send_event(&event);
                } else if(self._faultHandler == false) {
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
        if found == 0 && self._networkAccessController == true {
            error!("The component was not alive {}", component);
            self.publish_failure_component(system::constants::NETWORK_ACCESS_CONTROLLER);
            self._networkAccessController = false;
        } else if found == 0 && self._networkAccessController != true {
            debug!("The component is still dead {}", component);
        } else if found >= 1 && self._networkAccessController == false {
            warn!("The component is now alive {}", component);
            self._networkAccessController = true;
        } else {
            self._networkAccessController = true;
        }
    }

    fn check_environment_manager(&mut self) {
        let component = "exeEnvironmentManager";
        debug!("Looking for {}", component);
        let mut found = self._process.ps_find(&component); 
        while found == system::constants::ERROR_FIND {
            debug!("Find failed to run, retrying");
            found = self._process.ps_find(&component);
        }
        if found == 0 && self._environmentManager == true {
            error!("The component was not alive {}", component);
            self.publish_failure_component(system::constants::ENVIRONMENT_MANAGER);
            self._environmentManager = false;
        } else if found == 0 && self._environmentManager != true {
            debug!("The component is still dead {}", component);
        } else if found >= 1 && self._environmentManager == false {
            warn!("The component is now alive {}", component);
            self._environmentManager = true;
        } else {
            debug!("The component is alive {}", component);
            self._environmentManager = true;
        }      
    }

    fn check_database_manager(&mut self) {
        let component = "DatabaseManager";
        debug!("Looking for {}", component);
        let mut found = self._process.ps_find(&component); 
        while found == system::constants::ERROR_FIND {
            debug!("Find failed to run, retrying");
            found = self._process.ps_find(&component);
        }
        if found == 0 && self._databaseManager == true {
            error!("The component was not alive {}", component);
            self.publish_failure_component(system::constants::DATABASE_MANAGER);
            self._databaseManager = false;
            if self.check_file("/home/simon/Documents/Deploy/logs/DBM.txt") {
                if self.compare("/home/simon/Documents/Deploy/logs/DBM.txt", "Exception") {
                    error!("Log file exists and a exception occured");
                }
            }
        } else if found == 0 && self._databaseManager != true {
            debug!("The component is still dead {}", component);
        } else if found >= 1 && self._databaseManager == false {
            warn!("The component is now alive {}", component);
            self._databaseManager = true;
        } else {
            debug!("The component is alive {}", component);
            self._databaseManager = true;
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

    fn send_event(&mut self, message: &rabbitmq::types::EventSyp) {
        debug!("Publishing a event message about: {}", message.message);
        let serialized = serde_json::to_string(&message).unwrap();
        self._channel.publish(rabbitmq::types::EVENT_SYP, &serialized);
        self._event_counter += 1;
    }

    pub fn get_shutdown(&mut self) -> bool {
        return self._shutdown;
    }

    pub fn set_shutdown(&mut self) {
        self._shutdown = true;
    }

    pub fn get_event_counter(&mut self) -> u32 {
        return self._event_counter;
    }

    pub fn control_loop(&mut self) {
        trace!("Declaring consumer...");
        self._channel.consume();
        thread::sleep(time::Duration::from_secs(60));
        while self._shutdown != true {
            thread::sleep(time::Duration::from_secs(60));
            self.check_rabbitmq();
            self.check_fault_handler();
            thread::sleep(time::Duration::from_secs(60));
            self.check_sql();
            self.check_database_manager();
            self.check_environment_manager();
            self.check_network_access_controller();
            self.check_camera_monitor();
            self.check_user_panel();
            self.get_status_update();
        }
    }
}

fn main() {
    simple_logger::init_with_level(Level::Warn).unwrap();

    if log_enabled!(Level::Trace) {
        info!("Logging has been enabled to trace");
    }

    App::new("exeSystemProcessor")
        .version("1.2.0")
        .about("The hearbeat and monitor for HouseGuard.");

    let mut control = Control::new();

    control.control_loop();
    process::exit(0);
}
