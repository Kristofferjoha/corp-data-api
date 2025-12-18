mod utils;
use utils::clean_db;
use serial_test::serial;

use corp_data_api::entity::office::Office;
use corp_data_api::repository::office_repository::OfficeRepository;
use corp_data_api::config::db_settings::Settings;

/// 3 tests for repo layer, covers basic CRUD
/// Should obviously be made such that it covers everything


/// (C)ru(D) test for adding and removing office 
#[tokio::test]
#[serial]
async fn adding_removing_office_repo_test() {
    dotenv::from_filename(".env.test").ok();
    let pool = Settings::connect_from_env().unwrap().create_pool().await.unwrap();

    clean_db(&pool).await;

    let repo = OfficeRepository::new(pool.clone());

    let office = Office { id: None, name: "Test".into(), max_occupancy: 5 };
    let created = repo.create_office(&office).await.unwrap();

    let fetched = repo.get_office_by_id(created.id.unwrap()).await.unwrap();
    assert!(fetched.is_some());

    repo.delete_office(created.id.unwrap()).await.unwrap();

    let fetched_after_delete = repo.get_office_by_id(created.id.unwrap()).await.unwrap();
    assert!(fetched_after_delete.is_none());
    clean_db(&pool).await;
}

/// c(R)ud test for getting all offices
#[tokio::test]
#[serial]
async fn list_all_offices_repo_test() {
    dotenv::from_filename(".env.test").ok();
    let pool = Settings::connect_from_env().unwrap().create_pool().await.unwrap();

    clean_db(&pool).await;

    let repo = OfficeRepository::new(pool.clone());

    let office1 = Office { id: None, name: "OfficeUno".into(), max_occupancy: 5 };
    let office2 = Office { id: None, name: "OfficeDos".into(), max_occupancy: 10 };

    repo.create_office(&office1).await.unwrap();
    repo.create_office(&office2).await.unwrap();

    let offices = repo.get_all_offices().await.unwrap();
    assert_eq!(offices.len(), 2);
    clean_db(&pool).await;
}

/// cr(U)d test for updating office, this does not look at if employees exceed max occupancy after update
#[tokio::test]
#[serial]
async fn update_office_test() {
    dotenv::from_filename(".env.test").ok();
    let pool = Settings::connect_from_env().unwrap().create_pool().await.unwrap();

    clean_db(&pool).await;

    let repo = OfficeRepository::new(pool.clone());

    let office1 = Office { id: None, name: "OfficeUno".into(), max_occupancy: 5 };

    repo.create_office(&office1).await.unwrap();

    let mut fetched = repo.get_office_by_name("OfficeUno").await.unwrap().unwrap();
    fetched.name = "UpdatedOffice".into();
    fetched.max_occupancy = 15;
    repo.update_office_by_id(fetched.id.unwrap(), &fetched).await.unwrap();
    let updated = repo.get_office_by_id(fetched.id.unwrap()).await.unwrap().unwrap();
    assert_eq!(updated.name, "UpdatedOffice");
    assert_eq!(updated.max_occupancy, 15);

    clean_db(&pool).await;
}