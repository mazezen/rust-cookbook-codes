use std::println;

use sqlx::{Error, Row, postgres::PgPool, sqlite::SqlitePoolOptions};

async fn sqlx_do() -> Result<(), Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS location (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            latitude REAL NOT NULL,
            longitude REAL NOT NULL
        )
        ",
    )
    .execute(&pool)
    .await?;

    sqlx::query("INSERT INTO location (name, latitude, longitude) VALUES (?1, ?2, ?3)")
        .bind("Null Island")
        .bind(0.00)
        .bind(0.00)
        .execute(&pool)
        .await?;

    sqlx::query("INSERT INTO location (name, latitude, longitude) VALUES (?1, ?2, ?3)")
        .bind("Titanic")
        .bind(41.726931)
        .bind(49.948253)
        .execute(&pool)
        .await?;

    let locations = sqlx::query("SELECT name, latitude, longitude FROM location")
        .fetch_all(&pool)
        .await?;

    for row in locations {
        let name: String = row.try_get("name")?;
        let lat: f64 = row.try_get("latitude")?;
        let longitude: f64 = row.try_get("longitude")?;
        println!("{name} is at coordinates latitude: {lat} and longitude: {longitude}");
    }

    Ok(())
}

#[derive(sqlx::FromRow)]
struct Location {
    name: String,
    latitude: f64,
    longitude: f64,
}

async fn postgres_do() -> Result<(), Error> {
    let pool = PgPool::connect("postgresql://admin:admin@127.0.0.1:5433/postgres").await?;

    let locations = sqlx::query_as::<_, Location>("SELECT name, latitude, longitude FROM location")
        .fetch_all(&pool)
        .await?;

    for location in locations {
        println!(
            "{} is at coordinates latitude: {} and longitude: {}",
            location.name, location.latitude, location.longitude
        );
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    sqlx_do().await?;
    println!("----------------------------------------------");
    postgres_do().await?;
    println!("----------------------------------------------");
    Ok(())
}
