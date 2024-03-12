use std::{collections::VecDeque, option::Option, sync::{Arc, Mutex}, thread::{spawn, JoinHandle}, time::Duration};

pub trait Socket<T> {
    fn start(&mut self, events: Arc<Mutex<VecDeque<T>>>) -> ();
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

    fn valid_config(&self) -> Result<(), String> {
        Ok(())
    }
}

impl Socket<String> for Adapter {
    fn start(&mut self, events: Arc<Mutex<VecDeque<String>>>) -> () {
        let h = spawn(move || {
            loop {
                events.lock().unwrap().push_back("fake payload from socket".to_string());
                std::thread::sleep(Duration::from_millis(10));
            }
        });
        self.handle = Some(h);
    }

    fn stop(&mut self) -> Result<(), String> {
        match self.handle.take() {
            Some(handle) =>  handle.join().unwrap(),
            _ => return Err("failed to recover thread handle".to_string())
        }
        Ok(())
    }
}