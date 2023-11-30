# Axum REST API template

This is my template to use rust for backend REST APIs. For the core i used `axum` and for database connection I used `SQLx`.

## Setup DB with docker

1. Setup your ENV variables in `.env`
2. Run `docker-compose up` to start DB
3. Open `http://localhost:8080`, login and create required database
4. Run `cargo run`
5. Enjoy!