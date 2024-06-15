use std::collections::{VecDeque, HashMap};
use std::sync::{Arc, Condvar, Mutex};
use crate::{transceiver::Adapter, constants::{AdapterType, SOURCE_ADAPTERS}};
use crate::sensors::Pms7003SensorMeasurement;
use crate::transceiver::Socket;

const TAG: &'static str = "[interfaces]";

pub struct HardwareInterface {
    pub tag: String,
    pub settings: serde_json::Value,
    pub adapters: HashMap<AdapterType, (Adapter,  Arc<(Mutex<VecDeque<Pms7003SensorMeasurement>>, Condvar)>, Arc<Mutex<bool>>)>,
}

impl HardwareInterface {
    pub fn new(tag: String, settings: serde_json::Value) -> Self {
        let mut instance = Self {
            tag,
            settings,
            adapters: HashMap::new(),
        };
        instance.register_adapters();
        instance
    }

    pub fn start_adapter(&mut self, target: &AdapterType) -> Result<(), String> {
         let res = match self.adapters.get_mut(target) {
            Some(adapter) => adapter.0.start(adapter.1.clone(), adapter.2.clone()),
            _ =>  Err("target adapter not found - not started.".to_string())
        };
        res
    }

    pub fn start_adapters(&mut self) -> () {
        self.adapters.iter_mut().for_each(|(adapt_type, adapter_data)| {
            match adapter_data.0.start(adapter_data.1.clone(), adapter_data.2.clone()) {
                Ok(_) => println!("{} adapter: {} correctly started.", TAG, adapt_type.to_string()),
                _ => println!("{} adapter: {} failed to start.", TAG, adapt_type.to_string())
            }
        });
    }

    pub fn stop_adapter(&mut self, target: &AdapterType) -> Result<(), String> {
        let res = match self.adapters.get_mut(target) {
            Some(adapter) => adapter.0.stop(adapter.1.clone()),
            _ =>  Err("target adapter not found - not stopped.".to_string())
        };
        res
    }

    pub fn stop_adapters(&mut self) -> () {
        self.adapters.iter_mut().for_each(|(adapt_type, adapter_data)| {
            match adapter_data.0.stop(adapter_data.1.clone()) {
                Ok(_) => println!("{} adapter: {} correctly started.", TAG, adapt_type.to_string()),
                _ => println!("{} adapter: {} failed to stop.", TAG, adapt_type.to_string())
            }
        });
    }

    fn create_adapter(&mut self, adapter_type: AdapterType) -> () {
        let adapter = match adapter_type {
            AdapterType::Pms7003 => Some(Adapter::new(adapter_type.to_string(), self.settings["sensors"]["pms7003"].clone())),
            AdapterType::Dummy => Some(Adapter::new(adapter_type.to_string(), self.settings["sensors"]["dummy"].clone())),
        };
        self.adapters.insert(adapter_type, (
            adapter.expect("missing adapter"),
            Arc::new((Mutex::new(VecDeque::new()), Condvar::new())),
            Arc::new(Mutex::new(false)))
        );
        println!("{} created adapter: {}", self.tag, adapter_type.to_string());
    }

    fn register_adapters(&mut self) -> () {
        for adapter_type in SOURCE_ADAPTERS { 
            self.create_adapter(adapter_type);
        }
    }
}