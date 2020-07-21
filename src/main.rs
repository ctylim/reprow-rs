#[macro_use]
extern crate log;
pub mod config;
pub mod exec;
pub mod io;

use config::Config;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new();
    std::env::set_var("RUST_LOG", &config.log_level);
    env_logger::init();
    config.show();
    exec::exec(&config)?;
    Ok(())
}
