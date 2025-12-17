use crate::entity::office::Office;
use crate::repository::office_repository::OfficeRepository;
use anyhow::{anyhow};
use crate::utils::Validate;

/// Service for Office entities
/// Handles business logic related to offices
#[derive(Clone)]
pub struct OfficeService {
    repo: OfficeRepository,
}

impl OfficeService {
    /// Constructor for OfficeService
    pub fn new(repo: OfficeRepository) -> Self {
        Self { repo }
    }

    /// Adds a new office after validating and checking for duplicate names
    pub async fn add_office(&self, office: &Office) -> anyhow::Result<Office> {
        tracing::info!("Attempting to add office_id with name: {}", office.name);

        office.validate().map_err(|e| anyhow::anyhow!(e))?;

        if office.max_occupancy <= 0 {
            return Err(anyhow!("Max occupancy must be greater than 0"));
        }
        
        if self.repo.get_office_by_name(&office.name).await?.is_some() {
            return Err(anyhow::anyhow!("Office with name '{}' already exists", office.name));
        }

        self.repo.create_office(office).await
    }

    /// Finds an office by ID
    pub async fn find_office_by_id(&self, id: i32) -> anyhow::Result<Option<Office>> {
        tracing::info!("Attempting to find office with id: {}", id);
        self.repo.get_office_by_id(id).await
    }

    /// Lists all offices
    pub async fn list_all_offices(&self) -> anyhow::Result<Vec<Office>> {
        tracing::info!("Listing all offices");
        self.repo.get_all_offices().await
    }

    /// Updates an existing office after validating and checking for duplicate names
    pub async fn update_office(&self, id: i32, office: &Office) -> anyhow::Result<Office> {
        tracing::info!("Attempting to update office with id: {}", id);

        office.validate().map_err(|e| anyhow::anyhow!(e))?;

        if let Some(existing) = self.repo.get_office_by_name(&office.name).await? {
            if existing.id != Some(id) {
                return Err(anyhow::anyhow!("Name '{}' already taken", office.name));
            }
        }
        
        self.repo.update_office_by_id(id, office).await
    }

    /// Removes an office by ID
    pub async fn remove_office(&self, id: i32) -> anyhow::Result<bool> {
        tracing::info!("Deleting office id: {}", id);
        let rows = self.repo.delete_office(id).await?;
        Ok(rows > 0)
    }
}