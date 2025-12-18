<img width="1343" height="455" alt="image" src="https://github.com/user-attachments/assets/5fa3111c-05d3-4570-8f7b-8cc65e7c77fb" />

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
```powershell
cargo test
```
