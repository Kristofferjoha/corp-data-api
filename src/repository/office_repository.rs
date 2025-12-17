use crate::entity::office::Office;
use sqlx::PgPool;


#[derive(Clone)]
pub struct OfficeRepository {
    pool: PgPool,
}
impl OfficeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_office(&self, office: &Office) -> anyhow::Result<Office> {
        let created = sqlx::query_as!(
            Office,
            "INSERT INTO offices (name, max_occupancy) VALUES ($1, $2) RETURNING id, name, max_occupancy",
            office.name,
            office.max_occupancy
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(created)
    }

    pub async fn get_office_by_id(&self, id: i32) -> anyhow::Result<Option<Office>> {
        let office = sqlx::query_as!(
            Office,
            "SELECT id, name, max_occupancy FROM offices WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(office)
    }

    pub async fn get_all_offices(&self) -> anyhow::Result<Vec<Office>> {
        let offices = sqlx::query_as!(
            Office,
            "SELECT id, name, max_occupancy FROM offices"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(offices)
    }
    pub async fn get_office_by_name(&self, name: &str) -> anyhow::Result<Option<Office>> {
        let office = sqlx::query_as!(
            Office,
            "SELECT id, name, max_occupancy FROM offices WHERE name = $1",
            name
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(office)
    }

    pub async fn update_office_by_id(&self, id: i32, office: &Office) -> anyhow::Result<Office> {
        let updated = sqlx::query_as!(
            Office,
            "UPDATE offices SET name = $1, max_occupancy = $2 WHERE id = $3 RETURNING id, name, max_occupancy",
            office.name, 
            office.max_occupancy, 
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(updated)
    }

    pub async fn delete_office(&self, id: i32) -> anyhow::Result<u64> {
        let result = sqlx::query!("DELETE FROM offices WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }
}