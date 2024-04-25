use std::{collections::VecDeque, option::Option, sync::{Arc, Mutex, Condvar}, thread::{spawn, JoinHandle}, time::Duration};
use crate::sensors::DummySensor;

pub trait Socket<T> {
    fn validate_config(&mut self) -> Result<(), String>;

    fn start(&mut self, shared_data: Arc<(Mutex<VecDeque<T>>, Condvar)>, shutdown_request: Arc<Mutex<bool>>) -> Result<(), String>;
    fn stop(&mut self) -> Result<(), String>;
}

pub struct Adapter {
    pub name: String,
    pub settings: serde_json::Value,
    handle: Option<JoinHandle<()>>,
}

impl Adapter {
    pub fn new(name: String, settings: serde_json::Value) -> Self { 
        let handle = None;  
        Self {name, settings, handle}   
    }           
}

impl Socket<DummySensor> for Adapter {
    fn validate_config(&mut self) ->  Result<(), String> {
        Ok(())
    }

    fn start(&mut self, shared_data: Arc<(Mutex<VecDeque<DummySensor>>, Condvar)>, shutdown_request: Arc<Mutex<bool>>) -> Result<(), String> {
        match self.validate_config() {
            Ok(_) => println!("configuration correctly validated for adapter: {}", self.name),
            _ => return Err("failed to validate configuration for adapter: {} - start aborted.".to_string())
        }

        self.handle = Some(spawn(move || {
            loop {
                if *shutdown_request.lock().unwrap() {
                    break;
                }                
                std::thread::sleep(Duration::from_millis(500));
                shared_data.0.lock().unwrap().push_back(DummySensor{fake_payload: "fake payload from socket".to_string()});
                shared_data.1.notify_one();
            }
        })); 

        Ok(())
    }

    fn stop(&mut self) -> Result<(), String> {
        match self.handle.take() {
            Some(handle) =>  handle.join().expect(&format!("failed to join thread via handle for adapter: {}", self.name)),
            _ => return Err("failed to recover thread handle".to_string())
        }
        Ok(())
    }
}