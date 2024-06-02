use crate::{transceiver::Adapter, constants::{AdapterType, SOURCE_ADAPTERS}};

pub struct HardwareInterface {
    pub tag: String,
    pub settings: serde_json::Value,
    pub adapters: std::collections::HashMap<AdapterType, (Adapter, std::collections::VecDeque<Adapter>, bool)>,
}

impl HardwareInterface {
    pub fn new(tag: String, settings: serde_json::Value) -> Self {
        let mut instance = Self {
            tag,
            settings,
            adapters: std::collections::HashMap::new(),
        };
        instance.register_adapters();
        instance
    }

    pub fn request_adapter(&mut self, adapt_type: AdapterType) -> Option<&mut Adapter> {
        match self.adapters.get_mut(&adapt_type) {
            Some(adapter) => return Some(&mut adapter.0),
            _ => return None
        };
    }

    fn register_adapters(&mut self) -> () {
        for adapter_type in SOURCE_ADAPTERS { 
            self.create_adapter(adapter_type);
        }
    }

    fn create_adapter(&mut self, adapter_type: AdapterType) -> () {
       let adapter = match adapter_type {
            AdapterType::Pms7003 => Some(Adapter::new(adapter_type.to_string(), self.settings["sensors"]["pms7003"].clone())),
            AdapterType::Dummy => Some(Adapter::new(adapter_type.to_string(), self.settings["sensors"]["dummy"].clone())),
        };
        self.adapters.insert(adapter_type, (adapter.expect("missing adapter"), std::collections::VecDeque::new(), false));
        println!("{} created adapter: {}", self.tag, adapter_type.to_string());
    }
}