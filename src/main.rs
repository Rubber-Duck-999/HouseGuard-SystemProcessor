mod rabbitmq;
mod system;

use clap::App;

use std::process;

extern crate chrono;
use chrono::prelude::*;

#[macro_use]
extern crate log;
extern crate simple_logger;

use log::Level;

use std::collections::HashMap;
use std::path::Path;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

struct Control {
    _component_map: HashMap<u16, String>,
    _process: system::processes::Processes,
    _channel: rabbitmq::interaction::SessionRabbitmq,
    _shutdown: bool,
    _key: u16,
    _event_counter: u32,
}

impl Control {
    pub fn new() -> Control {
        Control {
            _component_map: HashMap::new(),
            _process: system::processes::Processes::new(),
            _channel: rabbitmq::interaction::SessionRabbitmq {
                ..Default::default()
            },
            _shutdown: false,
            _key: 0,
            _event_counter: 0,
        }
    }

    pub fn add_components_control(&mut self, component: &str, restart: bool) {
        trace!("Adding component to control map");
        if !self.start(component, restart) {
            error!(
                "The component will not start up, please debug {}",
                component
            );
        } else {
            self._component_map.insert(self._key, component.to_string()); // inserting moves `node`
            self._key += 1;
        }
    }

    pub fn add_components_shutdown(&mut self, component: &str) {
        trace!("Adding component to control map");
        let shell = system::constants::DEPLOY_SCRIPTS.to_owned() + &component.to_owned();
        let mut exists = Path::new(&shell).exists();
        warn!("Looking for {}, exist? {}", shell, exists);
        let mut found = self._process.ps_find(&shell);
        if found == 0 {
            error!("The component was not alive and we had a shutdown resuest, please debug {}", component);
            let event = rabbitmq::types::EventSyp {
                severity: 4,
                error: "Component not alive already - Request.Power".to_string(),
                time: self.get_time(),
                component: component.to_string(),
            };
            self.send_event(&event);
        } else {
            self._process.kill_component(&shell, false);
            found = self._process.ps_find(&component);
            if found > 0 {
                error!("The component will not die, please debug {}", component);
                let event = rabbitmq::types::EventSyp {
                    severity: 4,
                    error: "Component will not shutdown - Request.Power".to_string(),
                    time: self.get_time(),
                    component: system::constants::COMPONENT_NAME.to_string(),
                };
                self.send_event(&event);
                self._component_map.insert(self._key, component.to_string()); // inserting moves `node`
                self._key += 1;
            }
        }
    }

    fn get_time(&mut self) -> String {
        let dt = Utc.ymd(2020, 01, 01).and_hms(12, 0, 9);
        return dt.format("%Y-%m-%d %H:%M:%S").to_string();
    }

    fn switch_names(&mut self, component_name: &mut String) -> bool {
        let mut valid: bool = true;
        if component_name == system::constants::CAMERA_MONITOR {
            debug!("CM Found");
            let mut value = system::constants::CM_EXE.to_string();
            *component_name = value;
        } else if component_name == system::constants::NETWORK_ACCESS_CONTROLLER {
            debug!("NAC Found");
            let mut value = system::constants::NAC_EXE.to_string();
            *component_name = value;
        } else if component_name == system::constants::ENVIRONMENT_MANAGER {
            debug!("EVM Found");
            let mut value = system::constants::EVM_EXE.to_string();
            *component_name = value;
        } else if component_name == system::constants::FAULT_HANDLER {
            debug!("FH Found");
            let mut value = system::constants::FH_EXE.to_string();
            *component_name = value;
        } else if component_name == system::constants::DATABASE_MANAGER {
            debug!("DBM Found");
            let mut value = system::constants::DBM_EXE.to_string();
            *component_name = value;
        } else if component_name == system::constants::USER_PANEL {
            debug!("UP Found");
            let mut value = system::constants::UP_EXE.to_string();
            *component_name = value;
        } else if component_name == system::constants::COMPONENT_NAME {
            debug!("SYP Found - This initiates shutdown");
            self.set_shutdown();
        } else if component_name == system::constants::RABBITMQ {
            debug!("Rabbitmq Found");
        } else {
            debug!("Not sure what this is: {}", component_name);
            valid = false;
        }
        return valid;
    }

    fn exists_in_map(&mut self, component: &str) -> u8 {
        warn!("Looking for {}", component);
        let mut found: u8 = 0;
        for (key, val) in self._component_map.iter() {
            debug!("key: {}, name: {}", key, val);
            if val == component {
                found = found + 1;
            }
        }
        return found;
    }

    fn start(&mut self, component: &str, restart: bool) -> bool {
        let shell = system::constants::DEPLOY_SCRIPTS.to_owned() + &component.to_owned();
        let mut exists = Path::new(&shell).exists();
        warn!("Looking for {}, exist? {}", shell, exists);
        if exists {
            debug!(
                "The component file does exist: {}", exists);
            if restart {
                self._process.start_process(&shell);
                let mut found = self._process.ps_find(&shell);
                warn!("We have found {} processes for {}", found, &shell);
                if found > 1 {
                    self._process.kill_component(&shell, restart);
                    found = self._process.ps_find(&shell);
                } else if found != 1 {
                    let issue_pre = rabbitmq::types::IssueNotice {
                        severity: rabbitmq::types::START_UP_FAILURE_SEVERITY,
                        component: component.to_string(),
                        action: 0,
                    };
                    let issue = serde_json::to_string(&issue_pre).unwrap();
                    trace!("Serialized: {}", issue);
                    self._channel.publish(rabbitmq::types::ISSUE_NOTICE, &issue);
                    self._event_counter += 1;
                    exists = false;
                }
            }
        } else {
            let event = rabbitmq::types::EventSyp {
                severity: 5,
                error: "Component File Not Found".to_string(),
                time: self.get_time(),
                component: component.to_string(),
            };
            self.send_event(&event);
        }
        return exists;
    }

    fn send_event(&mut self, message: &rabbitmq::types::EventSyp) {
        warn!("Publishing a event message about: {}", message.error);
        let serialized = serde_json::to_string(&message).unwrap();
        self._channel
            .publish(rabbitmq::types::EVENT_SYP, &serialized);
        self._event_counter += 1;
    }

    fn request_check(&mut self, message: &mut rabbitmq::types::RequestPower) {
        let mut found: u8 = 0;
        warn!(
            "Power request for {} to be {}",
            message.component, message.power
        );
        let valid = self.switch_names(&mut message.component);
        if message.component != system::constants::COMPONENT_NAME {
            if valid {
                for (key, val) in self._component_map.iter() {
                    debug!("key: {}, name: {}", key, val);
                    if val.contains(&message.component) {
                        debug!("Found Component : {}", message.component);
                        found = found + 1;
                    }
                }
                if (found < 1) && (message.power == rabbitmq::types::RESTART) {
                    self.add_components_control(&mut message.component, rabbitmq::types::RESTART_SET);
                } else if message.power == rabbitmq::types::SHUTDOWN {
                    self.add_components_shutdown(&mut message.component);
                }
            } else {
                let event = rabbitmq::types::EventSyp {
                    severity: 2,
                    error: "Component Not Found- Request.Power".to_string(),
                    time: self.get_time(),
                    component: system::constants::COMPONENT_NAME.to_string(),
                };
                self.send_event(&event);
            }
        }
    }

    fn check_process(&mut self) {
        let failure = rabbitmq::types::FailureComponent {
            time: self.get_time(),
            type_of_failure: "Component died".to_string(),
            severity: rabbitmq::types::RUNTIME_FAILURE,
        };
        let mut found: u8 = 0;
        for (key, val) in self._component_map.iter() {
            debug!("key: {}, name: {}", key, val);
            let shell = system::constants::DEPLOY_SCRIPTS.to_owned() + &val.to_owned();
            if self._process.ps_find(&shell) < 1 {
                let serialized = serde_json::to_string(&failure).unwrap();
                warn!("Publishing a failure message: {}", serialized);
                self._channel
                    .publish(rabbitmq::types::FAILURE_COMPONENT, &serialized);
                self._event_counter += 1;
            }
        }
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

    fn control_loop(&mut self) {
        trace!("Declaring consumer...");
        self._channel.consume();
        let mut message = rabbitmq::types::RequestPower {
            power: rabbitmq::types::SHUTDOWN.to_string(),
            severity: 0,
            component: "None".to_string(),
        };

        while self._shutdown != true {
            if self._channel.consume_get(&mut message) {
                self.request_check(&mut message);
            }
            self.check_process();
        }
        trace!("Sending shutdown event...");
        let event = rabbitmq::types::EventSyp {
            severity: 6,
            error: "Component to be shutdown".to_string(),
            time: self.get_time(),
            component: system::constants::COMPONENT_NAME.to_string(),
        };
        self.send_event(&event);
    }
}

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    if log_enabled!(Level::Info) {
        info!("Logging has been enabled to info");
    }

    App::new("exeSystemProcessor")
        .version("0.0.1")
        .about("The hearbeat and starter for HouseGuard.");

    let mut control = Control::new();

    control.add_components_control(system::constants::FH_EXE, rabbitmq::types::RESTART_SET);

    control.add_components_control(system::constants::DBM_EXE, rabbitmq::types::RESTART_SET);

    control.add_components_control(system::constants::UP_EXE, rabbitmq::types::RESTART_SET);

    control.add_components_control(system::constants::NAC_EXE, rabbitmq::types::RESTART_SET);

    control.add_components_control(system::constants::CM_EXE, rabbitmq::types::RESTART_SET);

    control.add_components_control(system::constants::EVM_EXE, rabbitmq::types::RESTART_SET);

    control.control_loop();

    process::exit(0);
}
