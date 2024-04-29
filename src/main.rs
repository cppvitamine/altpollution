mod transceiver;
mod sensors;

use std::{collections::VecDeque, sync::{Arc, Condvar, Mutex}};
use serde_json::Value;
use crate::{sensors::Pms7003SensorMeasurement, transceiver::{Adapter, Socket}};

fn main() -> Result<(), String> {
    let tag: &str = "[main]";
    let sensor_config: &'static str = include_str!("../config/sensors.json");
    println!("{} reading configuration from ../config/sensors.json - config value {}", tag, sensor_config);

    let cfg: Value = match serde_json::from_str(sensor_config) {
        Ok(v) => v,
        _ => panic!("{} failed to load sensors configuration - program will exit now.", tag)
    };
    
    let shared_data: Arc<(std::sync::Mutex<VecDeque<Pms7003SensorMeasurement>>, Condvar)> = Arc::new((std::sync::Mutex::new(std::collections::VecDeque::new()), std::sync::Condvar::new()));
    let mut adapt: Adapter = Adapter::new("PMS7003".to_string(), cfg["sensors"]["pms7003"].clone());
    let shutdown_req = Arc::new(std::sync::Mutex::new(false));
    match adapt.start(shared_data.clone(), shutdown_req.clone()) {
        Ok(_) => println!("events queue for adapter: {} correctly forwared to adapter", adapt.name),
        _ => return Err("failed to share events queue for adatper".to_string())
    };

    loop {
        if shared_data.0.lock().unwrap().len() >= 10 {
            let mut shutdown = shutdown_req.lock().unwrap();
            *shutdown = true;
            break;
        }      
        println!("switching to passive wait on queue for adapter: {} to get events...", adapt.name);
        let _ = shared_data.1.wait(shared_data.0.lock().unwrap());
        println!("queue size: {}", shared_data.0.lock().unwrap().len());
        println!("{} received frame: {:?}", tag, shared_data.0.lock().unwrap().front().unwrap());
    }

    match adapt.stop(shared_data.clone()) {
        Ok(_) => println!("adapter: {} correctly stopped", adapt.name),
        _ => return Err("failed to stop adapter - quitting".to_string())
    };
    
    Ok(())
}
