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

fn start(component: &str) -> bool
{
    let mut process = system::processes::Processes::new();
    let mut exists = Path::new(component).exists();
    if exists
    {
        println!("The component file does exist: {}", 
            Path::new(component).exists());
        process.start_process(component);
        let mut found = process.ps_find(component);
        let mut restart = true;
        warn!("We have found {} processes for {}", found, component);
        let mut attempts = 0;
        while found != 1 && attempts < 3 
        {
            process.kill_component(component, restart);
            found = process.ps_find(component);
            attempts = attempts + 1;
        }
    }
    return exists;
}

fn control_process() 
{
    warn!(
        "Initialising System Processor Component = {}",
        system::constants::COMPONENT_NAME
    );

    let mut shutdown:bool = false;
    let mut channel = rabbitmq::interaction::SessionRabbitmq { ..Default::default() };
    
    let mut valid = start(system::constants::FH_EXE);
    start(system::constants::UP_EXE);
    start(system::constants::DBM_EXE);
    start(system::constants::NAC_EXE);
    start(system::constants::CM_EXE);
    start(system::constants::EVM_EXE);
    let issue_pre = rabbitmq::types::issue_notice 
            { _severity: rabbitmq::types::START_UP_FAILURE_SEVERITY
                , _component: "ALL".to_string(), _action: 0};
    let issue = serde_json::to_string(&issue_pre).unwrap();
    trace!("Serialized: {}", issue);
    channel.publish(rabbitmq::types::ISSUE_NOTICE, &issue);  


    trace!("Declaring consumer...");
    channel.Consume();
    while ! shutdown
    {
        channel.ConsumeGet();
        let mut event:&str = "{HELLO}";
        channel.publish(rabbitmq::types::EVENT_SYP, event);
    }
    trace!("Sending shutdown event...");
    let mut event:&str = "{HELLO}";
    channel.publish(rabbitmq::types::EVENT_SYP, event);  
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

    control_process();

    process::exit(0);
}
