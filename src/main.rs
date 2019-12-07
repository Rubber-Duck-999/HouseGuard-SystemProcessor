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

fn main()
{
    /*
    let matches = App::new("rust-rabbitmq-example")
        .version("0.0.1")
        .about("Simulate a RabbitMQ environment with consumer(s) and producer(s).")
        .arg(Arg::with_name("startup-script")
             .short("s")
             .long("startup-script")
             .help("Config file required by Component for startup settings")
             .takes_value(true)
        )
        .get_matches();

    let startup_script: usize = matches.value_of("startup-script")
        .unwrap_or("1")
        .parse()
        .unwrap();
    */
    simple_logger::init_with_level(Level::Warn).unwrap();


    if log_enabled!(Level::Info) {
        info!("Logging has been enabled to info");
    }

    warn!("Initialising System Processor Component = {}", system::constants::COMPONENT_NAME);

    let mut process_check = system::processes::Processes::new();

    process_check.start_process(system::constants::FAULT_HANDLER_EXE);
    let mut found = process_check.kill_component(system::constants::FAULT_HANDLER_EXE, false);

    let mut channel = rabbitmq::interaction::SessionRabbitmq { ..Default::default() };

    trace!("Declaring consumer...");
    channel.Consume();

    trace!("Declaring publish...");
    channel.publish(rabbitmq::types::ISSUE_NOTICE, system::constants::COMPONENT_NAME);

    process::exit(0);
}
