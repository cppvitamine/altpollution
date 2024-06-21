mod constants;
mod interfaces;
mod sensors;
mod transceiver;

use crate::{interfaces::HardwareInterface, constants::AdapterType};
use crate::sensors::Pms7003SensorMeasurement;

use prost::bytes::buf;
use prost::Message;

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
        Ok(_) => println!("{} PMS7003 sensor correctly started!", TAG),
        _ => return Err("Failed to start target PMS7003 sensor!".to_string()),
    }

    let adapt_target: AdapterType = AdapterType::Pms7003;
    let pms_events = intf.adapters[&adapt_target].1.clone();

    loop {
        let (lock, cvar) = &*pms_events;
        let mut queue = cvar.wait(lock.lock().unwrap()).unwrap();

        if let Some(received_frame) = queue.pop_front() {
            println!("{} PMS7003 frame received: {:?}", TAG, received_frame);
            let mut container: Vec<u8> = Vec::with_capacity(received_frame.encoded_len());
            match received_frame.encode(&mut container) {
                Ok(_) => {
                    println!("{} incoming PMS7003 frame correctly encoded: {:x?}", TAG, container);
                }
                Err(e) => {
                    println!("{} incoming PMS7003 frame encoding failed: {:?}", TAG, e);
                }
            }
        } else {
            println!("{} No frame to process", TAG);
        }
    }

    Ok(())
}
