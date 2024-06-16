mod constants;
mod interfaces;
mod sensors;
mod transceiver;

use crate::{interfaces::HardwareInterface, constants::AdapterType};

fn main() -> Result<(), String> {
    const TAG: &'static str = "[main]";
    let sensor_config: &'static str = include_str!("../config/sensors.json");
    println!("{} reading configuration from ../config/sensors.json - config value {}", TAG, sensor_config);

    let cfg: serde_json::Value = match serde_json::from_str(sensor_config) {
        Ok(v) => v,
        _ => panic!("{} failed to load sensors configuration - program will exit now.", TAG)
    };

    let mut intf: HardwareInterface = HardwareInterface::new("HW Interface".to_string(), cfg);
    match intf.start_adapter(&AdapterType::Pms7003) {
        Ok(_) => println!("PMS7003 sensor correctly started!"),
        _ => return Err("Failed to start target PMS7003 sensor!".to_string()),
    }
    
    Ok(())
}
 