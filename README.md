<img width="1370" height="444" alt="image" src="https://github.com/user-attachments/assets/8be6733a-0c60-45de-9bab-88b33396bc08" />


## Prereq
Docker og Rust

## Corp Data API 
Start postgreSQL database 

Lav environment variabler, brug ".prod.env" som skabelon

Logging:
```powershell
$env:RUST_LOG="info,corp_data_api=debug,sqlx=info";
```

Kør API
```powershell
cargo run
```

### Opret et office (Powershell)
```powershell
Invoke-RestMethod -Uri http://127.0.0.1:3000/offices `
  -Method Post `
  -ContentType "application/json" `
  -Body '{
    "name": "Aalborg",
    "max_occupancy": 3
  }'
```


## Tests
Indeholder grundlæggende tests alle lag:

Lav .env.test som indeholder:
```text
POSTGRES_USER=test_user
POSTGRES_PASSWORD=test_password
POSTGRES_DB=test
POSTGRES_HOST=localhost
POSTGRES_PORT=5432
```
```powershell
cargo test
```
