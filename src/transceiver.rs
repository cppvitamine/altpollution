use std::io;
//use serialport::{SerialPort, SerialPortType, DataBits, StopBits, Parity, FlowControl}; 
use serde_json::{Result, Value};

#[path = "../interfaces/generated/airquality.c.pb.rs"] mod airquality;

struct Receiver {
    setting: serde_json::Value
}
    
impl Receiver {
    // simulate payload from sensor
    pub fn fake_polling() -> String {
        "FAKE PAYLOAD".to_string()
    }
}