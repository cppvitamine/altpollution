use serde_json::{Result, Value};

mod transceiver;

fn main() -> () {
    // including needed config files
    let sensor_config: &'static str = include_str!("../config/sensors.json");
    println!("==== Welcome to AltPollution! ====");
}
