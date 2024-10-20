
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum AdapterType {
    Pms7003,
    Unknown,
}

impl AdapterType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Pms7003" => AdapterType::Pms7003,
            _ => AdapterType::Unknown,
        }
    }
}

impl std::fmt::Display for AdapterType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AdapterType::Pms7003 => write!(f, "Pms7003"),
            AdapterType::Unknown => write!(f, "Unknown"),
        }
    }
}


/* topics */
pub const PMS_7003_TOPIC: &str = "pms7003/readings";

pub const SOURCE_ADAPTERS: [AdapterType; 1] = [AdapterType::Pms7003];

