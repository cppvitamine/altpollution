
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum AdapterType {
    Pms7003,
    Dummy,
    Unknown,
}

impl AdapterType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Pms7003" => AdapterType::Pms7003,
            "Dummy" => AdapterType::Dummy,
            _ => AdapterType::Unknown,
        }
    }
}

impl std::fmt::Display for AdapterType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AdapterType::Pms7003 => write!(f, "Pms7003"),
            AdapterType::Dummy => write!(f, "Dummy"),
            AdapterType::Unknown => write!(f, "Unknown"),
        }
    }
}

pub const SOURCE_ADAPTERS: [AdapterType; 2] = [AdapterType::Pms7003, AdapterType::Dummy];