use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Office {
    id: u32,
    name: String,
    max_occupancy: u32,
}

impl Office {
    pub fn new(id: u32, name: String, max_occupancy: u32) -> Self {
        Office {
            id,
            name,
            max_occupancy,
        }
    }

    pub fn id(&self) -> u32 { self.id }
    pub fn name(&self) -> &str { &self.name }
    pub fn max_occupancy(&self) -> u32 { self.max_occupancy }
}