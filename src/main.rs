mod rabbitmq;
mod system;

use clap::{App, Arg};

use std::process;

use std::thread;
use std::time::Duration;

#[macro_use]
extern crate log;
extern crate simple_logger;

use log::Level;

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

struct Control
{
    _component_map: HashMap<u16, String>,
    _process: system::processes::Processes,
    _channel: rabbitmq::interaction::SessionRabbitmq,
    _shutdown: bool,
    _key: u16,
}

impl Control
{
    pub fn new() -> Control 
    {
        Control 
        {
            _component_map: HashMap::new(),
            _process : system::processes::Processes::new(),
            _channel : rabbitmq::interaction::SessionRabbitmq { ..Default::default() },
            _shutdown: false,
            _key: 0,
        }
    }

    pub fn add_components_control(&mut self, component: &str, restart: bool)
    {
        trace!("Adding component to control map");
        self._component_map.insert(self._key, component.to_string()); // inserting moves `node`
        self._key += 1;
        if ! self.start(component, restart)
        {
            error!("The component will not start up, please debug {}", component);
            process::exit(0);
        }
    }

    fn clear_map(&mut self) 
    {
        self._component_map.clear();
    }

    fn switch_names(&mut self, component_name:&mut String) -> bool
    {
        let mut valid:bool = true;
        if component_name == system::constants::CAMERA_MONITOR
        {
            debug!("CM Found");
        }
        else if component_name == system::constants::NETWORK_ACCESS_CONTROLLER
        {
            debug!("NAC Found");
        }
        else if component_name == system::constants::ENVIRONMENT_MANAGER
        {
            debug!("EVM Found");
        }
        else if component_name == system::constants::FAULT_HANDLER
        {
            debug!("FH Found");
        }
        else if component_name == system::constants::DATABASE_MANAGER
        {
            debug!("DBM Found");
        }
        else if component_name == system::constants::USER_PANEL
        {
            debug!("UP Found");
        }
        else if component_name == system::constants::COMPONENT_NAME
        {
            debug!("SYP Found");
            self._shutdown = true;
        }
        else if component_name == system::constants::RABBITMQ
        {
            debug!("NAC Found");
        }
        else
        {
            debug!("Not sure what this is: {}", component_name);
            valid = false;
        }
        return valid;
    }

    fn start(&mut self, component: &str, restart: bool) -> bool
    {
        let mut exists = Path::new(component).exists();
        if exists
        {
            debug!("The component file does exist: {}", 
                Path::new(component).exists());
            self._process.start_process(component);
            let mut found = self._process.ps_find(component);
            warn!("We have found {} processes for {}", found, component);
            let mut attempts = 0;
            while found != 1 && attempts < 3 
            {
                self._process.kill_component(component, restart);
                found = self._process.ps_find(component);
                attempts = attempts + 1;
            }
            if found != 1 
            {
                let issue_pre = rabbitmq::types::IssueNotice
                { 
                    severity: rabbitmq::types::START_UP_FAILURE_SEVERITY, 
                    component: component.to_string(), 
                    action: 0
                };
        
                let issue = serde_json::to_string(&issue_pre).unwrap();
                trace!("Serialized: {}", issue);
                self._channel.publish(rabbitmq::types::ISSUE_NOTICE, &issue); 
                exists = false;
            }
        }
        return exists;
    }

    fn request_check(&mut self, message:&mut rabbitmq::types::RequestPower)
    {
        let mut found:u8 = 0;
        warn!("Power request for {} to be {}", message.component, message.power);
        for (key, val) in self._component_map.iter() 
        {
            debug!("key: {}, name: {}", key, val);
            if val.contains(&message.component) 
            {
                debug!("Found Component : {}", message.component);
                found = found + 1;
            }
        }
        if self.switch_names(&mut message.component)
        {
            if ((found < 1) && (message.power == rabbitmq::types::RESTART))
            {
                self.add_components_control(&mut message.component,
                                            rabbitmq::types::RESTART_SET);
            }
            else if message.power == rabbitmq::types::SHUTDOWN
            {
                self.add_components_control(&mut message.component,
                                            rabbitmq::types::SHUTDOWN_SET);
            }
        }
        else
        {
            let event = rabbitmq::types::EventSyp
            { 
                severity: 2, 
                error: "Component Not Found- Request.Power".to_string(),
                time: "14:00:00".to_string(),
                component: system::constants::COMPONENT_NAME.to_string()
            };
            let serialized = serde_json::to_string(&event).unwrap();
            warn!("Serialized: {}", serialized);
            self._channel.publish(rabbitmq::types::EVENT_SYP, &serialized);
        }

    }

    fn check_process(&mut self)
    {
        let mut found:u8 = 0;
        for (key, val) in self._component_map.iter() 
        {
            debug!("key: {}, name: {}", key, val);
            if ! self._process.find(val)
            {
                let failure = rabbitmq::types::FailureComponent
                { 
                    time: "14:00:00".to_string(),
                    type_of_failure: "Component died".to_string(),
                    severity: rabbitmq::types::RUNTIME_FAILURE
                };
                let serialized = serde_json::to_string(&failure).unwrap();
                warn!("Serialized: {}", serialized);
                self._channel.publish(rabbitmq::types::FAILURE_COMPONENT, &serialized);
            }
        }
    }

    fn control_loop(&mut self) 
    {
        trace!("Declaring consumer...");
        self._channel.Consume();
        let mut message = rabbitmq::types::RequestPower
        {
            power: rabbitmq::types::SHUTDOWN.to_string(),
            severity: 0,
            component: "None".to_string()
        };

        while ! self._shutdown
        {
            if self._channel.ConsumeGet(&mut message)
            {
                self.request_check(&mut message);
            }
            self.check_process();
        }
        trace!("Sending shutdown event...");
        let mut event:&str = "{HELLO}";
        self._channel.publish(rabbitmq::types::EVENT_SYP, event);  
    }
}


fn main() 
{
    simple_logger::init_with_level(Level::Trace).unwrap();

    if log_enabled!(Level::Debug) 
    {
        info!("Logging has been enabled to info");
    }

    let matches = App::new("exeSystemProcessor")
        .version("0.0.1")
        .about("The hearbeat and starter for HouseGuard.");

    let mut control = Control::new();
    /*
    control.add_components_control(system::constants::FH_EXE, 
                                    rabbitmq::types::RESTART_SET);*/
    control.control_loop();

    process::exit(0);
}
