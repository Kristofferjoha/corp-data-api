use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Office {
    id: u32,
    name: String,
    max_occupancy: u32,
}

impl Office {
    pub fn new(id: u32, name: String, max_occupancy: u32) -> Result<Self> {
        if max_occupancy == 0 {
            return Err(anyhow!("max_occupancy must be at least 1"));
        }
        Ok(Self { id, name, max_occupancy })
    }


    pub fn id(&self) -> u32 { self.id }
    pub fn name(&self) -> &str { &self.name }
    pub fn max_occupancy(&self) -> u32 { self.max_occupancy }
}