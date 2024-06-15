use std::fmt;
use std::{collections::VecDeque, option::Option, sync::{Arc, Condvar, Mutex}, thread::{spawn, JoinHandle}, time::Duration};
use linux_embedded_hal;
use pms_7003::*;
use crate::sensors::{DummySensor, Pms7003SensorMeasurement};


pub trait Socket<T> {
    fn validate_config(&mut self) -> Result<(), String>;
    fn start(&mut self, shared_data: Arc<(Mutex<VecDeque<T>>, Condvar)>, shutdown_request: Arc<Mutex<bool>>) -> Result<(), String>;
    fn stop(&mut self, shared_data: Arc<(Mutex<VecDeque<T>>, Condvar)>) -> Result<(), String>;
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

impl Socket<Pms7003SensorMeasurement> for Adapter {
    fn validate_config(&mut self) ->  Result<(), String> {
        Ok(())
    }       

    fn start(&mut self, shared_data: Arc<(Mutex<VecDeque<Pms7003SensorMeasurement>>, Condvar)>, shutdown_request: Arc<Mutex<bool>>) -> Result<(), String> {
        match Socket::<Pms7003SensorMeasurement>::validate_config(self) {
            Ok(_) => println!("configuration correctly validated for adapter: {}", self.name),
            _ => return Err("failed to validate configuration - start aborted.".to_string())
        }
        
        let target_serial_path: String = self.settings["serial"].to_string();

        self.handle = Some(spawn(move || {
            let device = linux_embedded_hal::Serial::open(target_serial_path).expect("failed to retrieve device serial port path from configuration");
            let mut sensor = Pms7003Sensor::new(device);
            let mut max_retry: u8 = 20;
            loop {
                if *shutdown_request.lock().unwrap() {
                    break;
                }
                match sensor.read() {
                    Ok(frame) => {
                        let measurement =  Pms7003SensorMeasurement {
                            pm1_c_0: frame.pm1_0 as i32,
                            pm2_c_5: frame.pm2_5 as i32,
                            pm10: frame.pm10 as i32,
                            pm1_c_0_atm: frame.pm1_0_atm as i32,
                            pm2_c_5_atm: frame.pm2_5_atm as i32,
                            pm10_atm: frame.pm10_atm as i32,                      
                        };
                        shared_data.0.lock().unwrap().push_back(measurement);
                        shared_data.1.notify_one();
                    }
                    _ => {
                        max_retry -= 1;
                        if max_retry == 0 {
                            print!("[FATAL] failed to read PMS7003 sensor frame, no retry left - stopping adatper");
                            break;
                        }
                        print!("failed to read PMS7003 sensor frame, retry left: {}", max_retry);
                    }
                }
            }
        }));

        Ok(())
    }     

    fn stop(&mut self, shared_data: Arc<(Mutex<VecDeque<Pms7003SensorMeasurement>>, Condvar)>) -> Result<(), String> {
        match self.handle.take() {
            Some(handle) =>  handle.join().expect(&format!("failed to join thread via handle for adapter: {}", self.name)),
            _ => return Err("failed to recover thread handle".to_string())
        }
        println!("frames in queue after stop: {}", shared_data.0.lock().unwrap().len());
        Ok(())
    }
}

impl Socket<DummySensor> for Adapter {
    fn validate_config(&mut self) ->  Result<(), String> {
        Ok(())
    }

    fn start(&mut self, shared_data: Arc<(Mutex<VecDeque<DummySensor>>, Condvar)>, shutdown_request: Arc<Mutex<bool>>) -> Result<(), String> {
        match Socket::<DummySensor>::validate_config(self) {
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

    fn stop(&mut self, shared_data: Arc<(Mutex<VecDeque<DummySensor>>, Condvar)>) -> Result<(), String> {
        match self.handle.take() {
            Some(handle) =>  handle.join().expect(&format!("failed to join thread via handle for adapter: {}", self.name)),
            _ => return Err("failed to recover thread handle".to_string())
        }
        println!("frames in queue after stop: {}", shared_data.0.lock().unwrap().len());
        Ok(())
    }
}

impl fmt::Display for Pms7003SensorMeasurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {}, {}, {})", self.pm1_c_0, self.pm2_c_5, self.pm10, self.pm1_c_0_atm, self.pm2_c_5_atm, self.pm10_atm)
    }
}