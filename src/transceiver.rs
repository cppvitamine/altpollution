use std::fmt;
use std::{collections::VecDeque, option::Option, sync::{Arc, Condvar, Mutex}, thread::{spawn, JoinHandle}};
use linux_embedded_hal;
use pms_7003::*;
use prost::Message;
use unqlite::{UnQLite, KV};
use crate::{sensors::Pms7003SensorMeasurement, constants::PMS_7003_TOPIC};

pub trait Socket<T> {
    fn validate_config(&mut self) -> Result<(), String>;
    fn start(&mut self, shared_data: Arc<(Mutex<VecDeque<T>>, Condvar)>, shutdown_request: Arc<Mutex<bool>>) -> Result<(), String>;
    fn stop(&mut self, shared_data: Arc<(Mutex<VecDeque<T>>, Condvar)>) -> Result<(), String>;
}

pub struct Adapter {
    name: String,
    settings: serde_json::Value,
    storage: Arc<Mutex<UnQLite>>,
    producer: Option<JoinHandle<()>>,
    consumer: Option<JoinHandle<()>>
}

impl Adapter {
    pub fn new(name: String, settings: serde_json::Value, storage: Arc<Mutex<UnQLite>>) -> Self {
        let producer = None;
        let consumer = None;
        println!("adapter: {} received settings: {:?}", name, settings);
        Self {name, settings, storage, producer, consumer}
    }
}

impl Socket<Pms7003SensorMeasurement> for Adapter {
    fn validate_config(&mut self) ->  Result<(), String> {
        match self.settings.get("power_on") {
            Some(pwr) => {
                if !pwr.as_bool().unwrap() {
                    return Err("stopping sensor start() - power_on setting is false".to_string());
                }
            }
            _ => return Err("failed to validate configuration - missing power_on setting".to_string())
        }

        match self.settings.get("serial") {
            Some(serial) => {
                if serial.as_str().unwrap().is_empty() {
                    return Err("stopping sensor start() - missing target serial port".to_string());
                }
            }
            _ => return Err("failed to validate configuration - missing power_on setting".to_string())
        }

        println!("configuration correctly validated for adapter {}", self.name);
        Ok(())
    }

    fn start(&mut self, shared_data: Arc<(Mutex<VecDeque<Pms7003SensorMeasurement>>, Condvar)>, shutdown_request: Arc<Mutex<bool>>) -> Result<(), String> {
        match Socket::<Pms7003SensorMeasurement>::validate_config(self) {
            Ok(_) => println!("configuration correctly validated for adapter: {}", self.name),
            Err(e) => {
                println!("configuration validation failure reason: {}", e);
                return Err("failed to validate configuration - start aborted.".to_string())
            }
        }

        let target_serial_path: String = String::from(self.settings.get("serial").expect("failed to get target serial port").as_str().unwrap());
        println!("adapter: {} serial path: {}", self.name, target_serial_path);

        /*shared resources */
        let shared_data_prod = shared_data.clone();
        let shared_data_cons = shared_data.clone();

        let shutdown_producer = shutdown_request.clone();
        let shutdown_consumer = shutdown_request.clone();

        let storage = self.storage.clone();

        self.producer = Some(spawn(move || {
            let device = linux_embedded_hal::Serial::open(target_serial_path).expect("failed to retrieve device serial port path from configuration");
            let mut sensor = Pms7003Sensor::new(device);
            let mut max_retry: u8 = 20;
            let (lock_prod, cvar_prod) = &*shared_data_prod;
            loop {
                if *shutdown_producer.lock().unwrap() {
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
                        max_retry = 20;
                        lock_prod.lock().unwrap().push_back(measurement);
                        cvar_prod.notify_one();
                    }
                    _ => {
                        max_retry -= 1;
                        if max_retry == 0 {
                            println!("[FATAL] failed to read PMS7003 sensor frame, no retry left - stopping adapter");
                            break;
                        }
                        println!("failed to read PMS7003 sensor frame, retry left: {}", max_retry);
                    }
                }
            }
        }));

        self.consumer = Some(spawn(move || {
            let (lock_cons, cvar_cons) = &*shared_data_cons;
            loop {
                if *shutdown_consumer.lock().unwrap() {
                    break;
                }

                let mut queue = cvar_cons.wait(lock_cons.lock().unwrap()).unwrap();
                if let Some(rcv_evt) = queue.pop_front() {
                    println!("PMS7003 frame received: {:?}", rcv_evt);
                    let db = storage.lock().unwrap();

                    match db.kv_store(PMS_7003_TOPIC, rcv_evt.encode_to_vec()) {
                        Ok(_) => println!("PMS7003 frame stored successfully on topic: {}", PMS_7003_TOPIC),
                        Err(e) => println!("PMS7003 frame stored error: {}", e)
                    }
                } else {
                    println!("No frame to process");
                }
            }
        }));

        Ok(())
    }

    fn stop(&mut self, shared_data: Arc<(Mutex<VecDeque<Pms7003SensorMeasurement>>, Condvar)>) -> Result<(), String> {
        match self.producer.take() {
            Some(producer) =>  producer.join().expect(&format!("failed to join thread producer for adapter: {}", self.name)),
            _ => return Err("failed to recover thread producer".to_string())
        }
        match self.consumer.take() {
            Some(producer) =>  producer.join().expect(&format!("failed to join thread consumer for adapter: {}", self.name)),
            _ => return Err("failed to recover thread consumer".to_string())
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