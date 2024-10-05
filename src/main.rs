mod constants;
mod interfaces;
mod sensors;
mod transceiver;

use crate::{interfaces::HardwareInterface, constants::AdapterType};
use std::sync::atomic::{AtomicBool, Ordering};
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::flag;

fn main() -> Result<(), String> {
    let running = std::sync::Arc::new(AtomicBool::new(true));
    for sig in TERM_SIGNALS {
        flag::register(*sig, std::sync::Arc::clone(&running)).expect("Failed to register signal handler");
        flag::register_conditional_shutdown(*sig, 1, std::sync::Arc::clone(&running)).expect("Failed to register conditional handler");
    }

    const TAG: &'static str = "[main]";
    let sensor_config: &'static str = include_str!("../config/sensors.json");
    println!("{} reading configuration from ../config/sensors.json - config value {}", TAG, sensor_config);

    let cfg: serde_json::Value = match serde_json::from_str(sensor_config) {
        Ok(v) => v,
        _ => panic!("{} failed to load sensors configuration - program will exit now.", TAG)
    };

    println!("{} sensors configuration loaded: {:?}", TAG, cfg);

    let mut intf: HardwareInterface = HardwareInterface::new("HW Interface".to_string(), cfg);
    match intf.start_adapter(&AdapterType::Pms7003) {
        Ok(_) => println!("{} PMS7003 sensor correctly started!", TAG),
        Err(e) => panic!("{} failure to start target sensor Pms7003 reason: {} - program will exit now.", TAG, e)
    }

    while running.load(Ordering::Relaxed) {
        println!("{} heartbeat...", TAG);
        std::thread::sleep(std::time::Duration::from_millis(5000));
    }

    intf.start_adapters();
    Ok(())
}
