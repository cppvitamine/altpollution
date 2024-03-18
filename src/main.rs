mod transceiver;
mod sensors;

use std::{collections::VecDeque, fmt::format, sync::{Arc, Condvar, Mutex}};

use serde_json::Value;
use crate::{sensors::{Pms7003Sensor, DummySensor}, transceiver::{Adapter, Socket}};
use serial2::SerialPort;

fn main() -> Result<(), String> {
    let tag: &str = "[main]";
    let sensor_config: &'static str = include_str!("../config/sensors.json");
    println!("{} reading configuration from ../config/sensors.json - config value {}", tag, sensor_config);

    let cfg: Value = match serde_json::from_str(sensor_config) {
        Ok(v) => v,
        _ => panic!("{} failed to load sensors configuration - program will exit now.", tag)
    };
    
    let shared_data: Arc<(std::sync::Mutex<VecDeque<DummySensor>>, Condvar)> = Arc::new((std::sync::Mutex::new(std::collections::VecDeque::new()), std::sync::Condvar::new()));
    let mut adapt = Adapter::new("test".to_string(), cfg["dummy"].clone());
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
    }

    println!("{} received {} events for adapter: {}", tag, shared_data.0.lock().unwrap().len(), adapt.name);
    match adapt.stop() {
        Ok(_) => println!("adapter: {} correctly stopped", adapt.name),
        _ => return Err("failed to stop adapter - quitting".to_string())
    };
    
    let port = match SerialPort::open("/dev/tty.usbserial-A10OQMN5", 9600) {
        Ok(p) => p,
        Err(e) => return Err(format!("{} failed to open serial port due to error: {}", tag, e)),
        _ => return Err("unknown failure while opening serial port".to_string())
    };

    let mut buffer = [0; 256];
    for i in 0..10 {
        println!("{} reading buffer: {}", tag, i);
        let read = match port.read(&mut buffer) {
            Ok(r) => r,
            _ => return Err("failed to read from serial port".to_string())
        };

        if read == 0 {
            println!("{} no bytes read from serial port", tag);
        }
    }

    Ok(())
}