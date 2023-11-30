use axum::routing::post;
use axum::{routing::get, Router};
use clap::Parser;
use sqlx::postgres::PgPoolOptions;

use axum_rest_template::config::Config;

use axum_rest_template::user::router;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Load the .env file
    dotenv::dotenv().ok();

    // Parse the command line arguments
    let config = Config::parse();
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/user", post(router::create_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
