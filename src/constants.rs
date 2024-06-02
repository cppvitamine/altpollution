
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum AdapterType {
    Pms7003,
    Dummy,
}

impl std::fmt::Display for AdapterType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AdapterType::Pms7003 => write!(f, "Pms7003"),
            AdapterType::Dummy => write!(f, "Dummy"),
        }
    }
}

pub const SOURCE_ADAPTERS: [AdapterType; 2] = [AdapterType::Pms7003, AdapterType::Dummy];