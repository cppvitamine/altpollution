use std::sync::Arc;

use queues::Queue;
use serde_json::{Result, Value};
use crate::sensors::{DummySensor, Pms7003Sensor};

#[derive(Clone)]
enum SensorData {
    DummySensor,
    Pms7003Sensor,
}

pub trait Socket<SensorData> {
    fn start_polling(&self) -> ();
    fn get_message() -> Option<SensorData>;
}

pub struct Adapter {
    pub name: String,
    pub settings: serde_json::Value,
    data: Queue<SensorData>,
}

impl Adapter {
    pub fn new(name: String, settings: serde_json::Value, data: Queue<SensorData>) -> Self { 
        Self {name, settings, data}                       
    }
}

impl Socket<DummySensor> for Adapter {
    fn start_polling(&self) -> () {
        
    }

    fn get_message() -> Option<DummySensor> {
        let ds: DummySensor = DummySensor::new("fake payload".to_string());
        Some(ds)
    }
}
