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

    process::exit(0);
}
