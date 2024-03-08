use std::{borrow::Borrow, io};
use serde_json::{Result, Value};

pub trait Socket<T> {
    fn poll(&self) -> Option<T>;
}

pub trait Forwarder {
    fn send(&self) -> Result<()>;
}

pub struct Adapter {
    name: String,
    settings: serde_json::Value
}

impl Adapter {
    pub fn new(name: String, settings: serde_json::Value) -> Self { 
        Self {name, settings}                       
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_settings(&self) -> String {
        self.settings.clone().to_string()
    }
}

pub struct PMSAdapter {
    adapter: Adapter
}

pub struct DummyAdapter {
    adapter: Adapter
}