mod utils;
use utils::clean_db;
use serial_test::serial;

use corp_data_api::entity::{office::Office, employee::Employee};
use corp_data_api::repository::{office_repository::OfficeRepository, employee_repository::EmployeeRepository};
use corp_data_api::config::db_settings::Settings;
use corp_data_api::service::employee_service::EmployeeService;

/// Tests service layer
/// Should cover everything in service layer but only some basics are tested


/// Add two users to office of space 1, expects error for second entry
#[tokio::test]
#[serial]
async fn employee_office_max_occ_test() {
    dotenv::from_filename(".env.test").ok();
    let pool = Settings::connect_from_env().unwrap().create_pool().await.unwrap();
    clean_db(&pool).await;

    let office_repo = OfficeRepository::new(pool.clone());
    let employee_repo = EmployeeRepository::new(pool.clone());
    let service = EmployeeService::new(employee_repo.clone(), office_repo.clone());

    let office = Office { id: None, name: "Vester Hassing".into(), max_occupancy: 1 };
    let office_created = office_repo.create_office(&office).await.unwrap();

    let emp1 = Employee { id: None, first_name: "Kristoffer".into(), last_name: "Første".into(), birth_date: chrono::NaiveDate::from_ymd_opt(1950, 1, 1).expect("Invalid date"), office_id: office_created.id.unwrap() };
    service.add_employee(&emp1).await.unwrap();

    let emp2 = Employee { id: None, first_name: "Kristoffer".into(), last_name: "Anden".into(), birth_date: chrono::NaiveDate::from_ymd_opt(1950, 12, 23).expect("Invalid date"), office_id: office_created.id.unwrap() };
    let res = service.add_employee(&emp2).await;
    assert!(res.is_err());

    clean_db(&pool).await;
}

/// List employees by office id, expects 2 employees for created office, error for non existent
#[tokio::test]
#[serial]
async fn list_employees_by_office_service_test() {
    dotenv::from_filename(".env.test").ok();
    let pool = Settings::connect_from_env().unwrap().create_pool().await.unwrap();
    clean_db(&pool).await;

    let office_repo = OfficeRepository::new(pool.clone());
    let employee_repo = EmployeeRepository::new(pool.clone());
    let service = EmployeeService::new(employee_repo.clone(), office_repo.clone());

    let office = Office { id: None, name: "TestOffice".into(), max_occupancy: 5 };
    let office_created = office_repo.create_office(&office).await.unwrap();

    let emp1 = Employee { id: None, first_name: "Kristoffer".into(), last_name: "Første".into(), birth_date: chrono::NaiveDate::from_ymd_opt(1950, 1, 1).expect("Invalid date"), office_id: office_created.id.unwrap() };
    let emp2 = Employee { id: None, first_name: "Kristoffer2".into(), last_name: "Anden".into(), birth_date: chrono::NaiveDate::from_ymd_opt(1950, 12, 23).expect("Invalid date"), office_id: office_created.id.unwrap() };

    service.add_employee(&emp1).await.unwrap();
    service.add_employee(&emp2).await.unwrap();

    let employees = service.list_employees_by_office_id(office_created.id.unwrap()).await.unwrap();
    assert_eq!(employees.len(), 2);
    assert!(employees.iter().any(|e| e.first_name == "Kristoffer"));
    assert!(employees.iter().any(|e| e.first_name == "Kristoffer2"));

    let result = service.list_employees_by_office_id(999333).await;
    assert!(result.is_err());

    clean_db(&pool).await;
}