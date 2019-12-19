mod system;
mod rabbitmq;

use clap::{
    App,
    Arg,
};

use std::{process};

use std::thread;
use std::time::Duration;

#[macro_use]
extern crate log;
extern crate simple_logger;

use log::Level;

use std::fs::File;



fn start(component:&str)
{
    let mut process = system::processes::Processes::new();
    process.start_process(component);
    let mut found = process.ps_find(component);
    let mut restart = true;
    let mut attempts = 0;
    while found != 1 && attempts < 3
    {
        process.kill_component(component, restart);
        found = process.ps_find(component);
        attempts = attempts + 1;
    }
}

fn control_process()
{
    warn!("Initialising System Processor Component = {}", system::constants::COMPONENT_NAME);

    start(system::constants::FH_EXE);
    /*
    let mut channel = rabbitmq::interaction::SessionRabbitmq { ..Default::default() };

    trace!("Declaring consumer...");
    channel.Consume();
    while true 
    {
        channel.ConsumeGet();
    }
    trace!("Declaring startup event...");
    let mut event:&str = "{HELLO}";
    channel.publish(rabbitmq::types::EVENT_SYP, event);
    */
}

fn main()
{
    simple_logger::init_with_level(Level::Trace).unwrap();

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
