use std::io;
use serde_json::{Result, Value};

struct Receiver {
    setting: serde_json::Value
}
    
impl Receiver {
    // simulate payload from sensor
    pub fn fake_polling() -> String {
        "FAKE PAYLOAD".to_string()
    }
}