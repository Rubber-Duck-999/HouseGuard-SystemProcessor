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

use std::path::Path;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate system_shutdown;

use system_shutdown::shutdown;

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
            _userPanel: false,
            _faultHandler: false,
            _databaseManager: false,
            _cameraMonitor: false,
            _environmentManager: false,
            _networkAccessController: false,
            _sql: false,
            _rabbitmq: false,
        }
    }

    fn self.publish_failure_component(&mut self, component: &str) -> bool {
        if self._rabbitmq == false {
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

    fn check_user_panel(&mut self) {
        let component = "UserPanel";
        debug!("Looking for {}", component);
        let mut found = self._process.ps_find(&component); 
        while found == system::constants::ERROR_FIND {
            error!("Find failed to run, retrying");
            found = self._process.ps_find(&component);
        }
        if found == 0 && self._userPanel == true {
            error!("The component was not alive, please debug {}", component);
            self.publish_failure_component(system::constants::USER_PANEL);
            self._userPanel = false;
        } else if found == 0 && self._userPanel != true {
            warn!("The component is still dead {}", component);
        }
        else {
            self._userPanel = true;
        }
    }

    fn check_camera_monitor(&mut self) {
        let component = "exeCameraMonitor";
        debug!("Looking for {}", component);
        let mut found = self._process.ps_find(&component); 
        while found == system::constants::ERROR_FIND {
            error!("Find failed to run, retrying");
            found = self._process.ps_find(&component);
        }
        if found == 0 && self._cameraMonitor == true {
            error!("The component was not alive, please debug {}", component);
            self.publish_failure_component(system::constants::CAMERA_MONITOR);
            self._cameraMonitor = false;
        } else if found == 0 && self._cameraMonitor != true {
            warn!("The component is still dead, please debug {}", component);
        }
        else {
            self._cameraMonitor = true;
        }      
    }

    fn check_fault_handler(&mut self) {
        let component = "exeFaultHandler";
        debug!("Looking for {}", component);
        let mut found = self._process.ps_find(&component); 
        while found == system::constants::ERROR_FIND {
            error!("Find failed to run, retrying");
            found = self._process.ps_find(&component);
        }
        if found == 0 {
            error!("Fault handler is not alive, we should restart the system");
            warn!("Rerunning find to ensure its definitely not a failure");
            if(self._process.ps_find(&component) == 0)
            {
                self._faultHandler = false;
                match reboot() {
                    Ok(_) => println!("Rebooting ..."),
                    Err(error) => eprintln!("Failed to reboot: {}", error),
                }
            }
        }
    }

    fn check_network_access_controller(&mut self) {
        let component = "exeNetworkAccessController";
        debug!("Looking for {}", component);
        let mut found = self._process.ps_find(&component); 
        while found == system::constants::ERROR_FIND {
            error!("Find failed to run, retrying");
            found = self._process.ps_find(&component);
        }
        if found == 0 && self._networkAccessController == true {
            error!("The component was not alive, please debug {}", component);
            self.publish_failure_component(system::constants::NETWORK_ACCESS_CONTROLLER);
            self._networkAccessController = false;
        } else if found == 0 && self._networkAccessController != true {
            warn!("The component is still dead {}", component);
        }
        else {
            self._networkAccessController = true;
        }
        return self._networkAccessController;
    }

    fn check_environment_manager(&mut self) {
        let component = "exeEnvironmentManager";
        debug!("Looking for {}", component);
        let mut found = self._process.ps_find(&component); 
        while found == system::constants::ERROR_FIND {
            error!("Find failed to run, retrying");
            found = self._process.ps_find(&component);
        }
        if found == 0 && self._environmentManager == true {
            error!("The component was not alive, please debug {}", component);
            self.publish_failure_component(system::constants::ENVIRONMENT_MANAGER);
            self._environmentManager = false;
        } else if found == 0 && self._environmentManager != true {
            warn!("The component is still dead {}", component);
        }
        else {
            debug!("The component is alive {}", component);
            self._environmentManager = true;
        }      
    }

    fn check_database_manager(&mut self) {
        let component = "DatabaseManager";
        debug!("Looking for {}", component);
        let mut found = self._process.ps_find(&component); 
        while found == system::constants::ERROR_FIND {
            error!("Find failed to run, retrying");
            found = self._process.ps_find(&component);
        }
        if found == 0 && self._databaseManager == true {
            error!("The component was not alive, please debug {}", component);
            self.publish_failure_component(system::constants::DATABASE_MANAGER);
            self._databaseManager = false;
        } else if found == 0 && self._databaseManager != true {
            warn!("The component is still dead {}", component);
        }
        else {
            debug!("The component is alive {}", component);
            self._databaseManager = true;
        }      
    }

    fn check_sql(&mut self) {
        let component = "mysql";
        debug!("Looking for {}", component);
        let mut found = self._process.ps_find(&component); 
        while found == system::constants::ERROR_FIND {
            error!("Find failed to run, retrying");
            found = self._process.ps_find(&component);
        }
        if found == 0 && self._sql == true {
            error!("The component was not alive, please debug {}", component);
            self.publish_failure_component(system::constants::SQL);
            self._sql = false;
        } else if found == 0 && self._sql != true {
            warn!("The component is still dead {}", component);
        }
        else {
            debug!("The component is alive {}", component);
            self._sql = true;
        }
    }

    fn check_rabbitmq(&mut self) {
        let component = "rabbitmq";
        debug!("Looking for {}", component);
        let mut found = self._process.ps_find(&component); 
        while found == system::constants::ERROR_FIND {
            error!("Find failed to run, retrying");
            found = self._process.ps_find(&component);
        }
        if found == 0 && self._rabbitmq == true {
            error!("The component was not alive, please debug {}", component);
            self.publish_failure_component(system::constants::SQL);
            self._rabbitmq = false;
        } else if found == 0 && self._rabbitmq != true {
            warn!("The component is still dead {}", component);
        }
        else {
            debug!("The component is alive {}", component);
            self._rabbitmq = true;
        }
    }

    fn send_event(&mut self, message: &rabbitmq::types::EventSyp) {
        warn!("Publishing a event message about: {}", message.message);
        let serialized = serde_json::to_string(&message).unwrap();
        self._channel.publish(rabbitmq::types::EVENT_SYP, &serialized);
        self._event_counter += 1;
    }

    fn control_loop(&mut self) {
        trace!("Declaring consumer...");
        self._channel.consume();
        thread::sleep(time::Duration::from_secs(5));
        while self._shutdown != true {
            thread::sleep(time::Duration::from_secs(5));
            self.check_rabbitmq();
            self.check_fault_handler();
            thread::sleep(time::Duration::from_secs(5));
            self.check_sql();
            self.check_database_manager();
            self.check_environment_manager();
            self.check_network_access_controller();
            self.check_camera_monitor();
            self.check_user_panel();
        }
    }
}

fn main() {
    simple_logger::init_with_level(Level::Warn).unwrap();

    if log_enabled!(Level::Trace) {
        info!("Logging has been enabled to info");
    }

    App::new("exeSystemProcessor")
        .version("1.2.0")
        .about("The hearbeat and monitor for HouseGuard.");

    let mut control = Control::new();

    control.control_loop();

    process::exit(0);
}
