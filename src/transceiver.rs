use serde_json::{Result, Value};

pub trait Socket<T> {
    fn poll(&self) -> Option<T>;
}

pub trait Forwarder {
    fn send(&self) -> Result<()>;
}

pub struct Adapter {
    pub name: String,
    pub settings: serde_json::Value
}

impl Adapter {
    pub fn new(name: String, settings: serde_json::Value) -> Self { 
        Self {name, settings}                       
    }
}

pub struct PMSAdapter {
    pub adapter: Adapter
}

pub struct DummyAdapter {
    pub adapter: Adapter
}