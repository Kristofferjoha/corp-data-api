use crate::entity::office::Office;
use crate::repository::office_repository::OfficeRepository;
use anyhow::{anyhow};

#[derive(Clone)]
pub struct OfficeService {
    repo: OfficeRepository,
}

impl OfficeService {
    pub fn new(repo: OfficeRepository) -> Self {
        Self { repo }
    }

    pub async fn add_office(&self, office: &Office) -> anyhow::Result<Office> {
        tracing::info!("Attempting to add office_id with name: {}", office.name);
        if office.max_occupancy <= 0 {
            return Err(anyhow!("Max occupancy must be greater than 0"));
        }
        
        if self.repo.get_office_by_name(&office.name).await?.is_some() {
            return Err(anyhow::anyhow!("Office with name '{}' already exists", office.name));
        }

        self.repo.create_office(office).await
    }

    pub async fn find_office_by_id(&self, id: i32) -> anyhow::Result<Option<Office>> {
        tracing::info!("Attempting to find office with id: {}", id);
        self.repo.get_office_by_id(id).await
    }

    pub async fn list_all_offices(&self) -> anyhow::Result<Vec<Office>> {
        tracing::info!("Listing all offices");
        self.repo.get_all_offices().await
    }
}