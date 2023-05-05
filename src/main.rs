use axum::{routing::get, Router};
use migration::{sea_orm, Migrator, MigratorTrait};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let database_url: String =
        "postgres://postgres:toujours_skateboarding@localhost:5432/toujours_skateboarding_axum_db"
            .to_owned();

    let connection = sea_orm::Database::connect(&database_url)
        .await
        .expect("Connection impossible");

    Migrator::up(&connection, None)
        .await
        .expect("Migration error");

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    axum::Server::bind(&"0.0.0.0:7878".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
