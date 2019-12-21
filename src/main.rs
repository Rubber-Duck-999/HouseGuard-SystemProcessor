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
    _key: 16,
}

impl Control
{
    pub fn new() -> Control 
    {
        Control 
        {
            _component_map: HashMap::new(),
            _process = system::processes::Processes::new(),
            _channel = rabbitmq::interaction::SessionRabbitmq { ..Default::default() };
            _shutdown: false,
            _key: 0,
        }
    }

    pub fn add_components_control(component: &str)
    {
        trace!("Adding found component to control map");
        self.component_map.insert(self._key, component); // inserting moves `node`
        self.id_key += 1;
        if ! start(compnent)
        {
            warn!("The component will not start up, please debug {}", component);
        }
    }

    fn clear_map(&mut self) 
    {
        self.component_map.clear();
    }

    fn start&mut self, component: &str) -> bool
    {
        //let mut process = system::processes::Processes::new();
        let mut exists = Path::new(component).exists();
        if exists
        {
            println!("The component file does exist: {}", 
                Path::new(component).exists());
            self._process.start_process(component);
            let mut found = self._process.ps_find(component);
            let mut restart = true;
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
                let issue_pre = rabbitmq::types::issue_notice
                { 
                    severity: rabbitmq::types::START_UP_FAILURE_SEVERITY, 
                    component: component.to_string(), 
                    action: 0
                };
        
                let issue = serde_json::to_string(&issue_pre).unwrap();
                trace!("Serialized: {}", issue);
                channel.publish(rabbitmq::types::ISSUE_NOTICE, &issue); 
                exists = false;
            }
        }
        return exists;
    }

    fn request_check(message:&mut types::request_power)
    {

    }

    fn control_loop() 
    {
        trace!("Declaring consumer...");
        channel.Consume();
        let mut message = rabbitmq::types::request_power
        {
            power: rabbitmq::types::SHUTDOWN.to_string(),
            severity: 0,
            component: "None".to_string()
        };

        while ! shutdown
        {
            if channel.ConsumeGet(&mut message)
            {
                request_check(&mut message, &mut channel);
            }
            let mut event:&str = "{HELLO}";
            channel.publish(rabbitmq::types::EVENT_SYP, event);
        }
        trace!("Sending shutdown event...");
        let mut event:&str = "{HELLO}";
        channel.publish(rabbitmq::types::EVENT_SYP, event);  
    }
}


fn main() 
{
    simple_logger::init_with_level(Level::Warn).unwrap();

    if log_enabled!(Level::Info) 
    {
        info!("Logging has been enabled to info");
    }

    let matches = App::new("exeSystemProcessor")
        .version("0.0.1")
        .about("The hearbeat and starter for HouseGuard.");

    let mut control = Control::new();
    Control.add_components_control()
    Control.control_loop(system::constants::FH_EXE);

    process::exit(0);
}
