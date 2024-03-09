use serde_json::{Result, Value};
mod transceiver;
use crate::transceiver::Adapter;

fn main() -> () {
    let tag: &str = "[main]";
    let sensor_config: &'static str = include_str!("../config/sensors.json");
    println!("{} reading configuration from ../config/sensors.json - config value {}", tag, sensor_config);

    let cfg: Value = match serde_json::from_str(sensor_config) {
        Ok(v) => v,
        _ => panic!("{} failed to load sensors configuration - program will exit now.", tag)
    };
 
    let dummy_adapter: Adapter = Adapter::new("dummy_adapter".to_string(), cfg["dummy"].to_owned());
}
