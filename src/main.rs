mod transceiver;
mod sensors;

use serde_json::Value;
use crate::{sensors::{Pms7003Sensor, DummySensor}, transceiver::{Adapter, Socket}};

fn main() -> () {
    let tag: &str = "[main]";
    let sensor_config: &'static str = include_str!("../config/sensors.json");
    println!("{} reading configuration from ../config/sensors.json - config value {}", tag, sensor_config);

    let cfg: Value = match serde_json::from_str(sensor_config) {
        Ok(v) => v,
        _ => panic!("{} failed to load sensors configuration - program will exit now.", tag)
    };
    
    let queue = std::sync::Arc::new(std::sync::Mutex::new(std::collections::VecDeque::new()));
    let mut adapt = Adapter::new("test".to_string(), cfg["dummy"].clone());
    adapt.start(queue.clone());

    loop {
        if queue.lock().unwrap().len() >= 10 {
            break;
        }
        println!("queue: {:?}", queue.lock().unwrap());
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
