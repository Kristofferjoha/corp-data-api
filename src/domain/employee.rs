use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub id: u32,
    pub name: String,
    pub office_id: u32,
}

impl Employee {
    pub fn new(id: u32, name: String, office_id: u32) -> Self {
        Employee {
            id,
            name,
            office_id,
        }
    }

    pub fn id(&self) -> u32 { self.id }
    pub fn name(&self) -> &str { &self.name }
    pub fn office_id(&self) -> u32 { self.office_id }
}