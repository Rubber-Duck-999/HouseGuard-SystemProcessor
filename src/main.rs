use clap::App;

use std::process;

#[macro_use]
extern crate log;
extern crate simple_logger;

use log::Level;

fn main() {
    simple_logger::init_with_level(Level::Trace).unwrap();

    if log_enabled!(Level::Trace) {
        info!("Logging has been enabled to trace");
    }

    App::new("exeSystemProcessor")
        .version("1.4.0")
        .about("The hearbeat and monitor for HouseGuard.");

    process::exit(0);
}
