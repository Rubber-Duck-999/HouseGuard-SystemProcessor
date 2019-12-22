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

    fn switch_names(&mut self, component_name: &str) 
    {
        let result = component_name;
        let name = match result
        {
            system::constants::CAMERA_MONITOR => {
                compoment_name = system::constants::CM_EXE;
            }
            _ => {
                component = system::constants::FH_EXE;
            }
        };
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
        if ((found < 1) && (message.power == rabbitmq::types::RESTART))
        {
            self.switch_names(&mut message.compoment);
            self.add_components_control(&mut message.component,
                                        rabbitmq::types::RESTART_SET);
        }
        else if message.power == rabbitmq::types::SHUTDOWN
        {
            self.switch_names(&mut message.compoment);
            self.add_components_control(&mut message.component,
                                        rabbitmq::types::SHUTDOWN_SET);
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
            let mut event:&str = "{HELLO}";
            self._channel.publish(rabbitmq::types::EVENT_SYP, event);
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
    /*control.add_components_control(system::constants::FH_EXE, 
                                    rabbitmq::types::RESTART_SET);*/
    control.control_loop();

    process::exit(0);
}
