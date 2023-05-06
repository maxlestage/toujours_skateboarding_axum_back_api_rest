use axum::{
    extract,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use entity::prelude::User;
use from_body::UserSerialized::UserSerialized;
use migration::{sea_orm, Migrator, MigratorTrait};
use querry::user::create_new_user;

async fn create_user(Json(payload): Json<UserSerialized>) -> (StatusCode, Json<UserSerialized>) {
    let database_url: String =
        "postgres://postgres:toujours_skateboarding@localhost:5432/toujours_skateboarding_axum_db"
            .to_owned();

    let connection: sea_orm::DatabaseConnection = sea_orm::Database::connect(&database_url)
        .await
        .expect("Connection impossible");
    let user: UserSerialized = UserSerialized {
        username: payload.username,
        firstname: payload.firstname,
        lastname: payload.lastname,
        cellnumber: payload.cellnumber,
        mail: payload.mail,
        password: payload.password,
        secretkey: payload.secretkey,
    };

    create_new_user(user.clone(), &connection).await;
    (StatusCode::CREATED, Json(user))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let database_url: String =
        "postgres://postgres:toujours_skateboarding@localhost:5432/toujours_skateboarding_axum_db"
            .to_owned();

    let connection: sea_orm::DatabaseConnection = sea_orm::Database::connect(&database_url)
        .await
        .expect("Connection impossible");

    Migrator::up(&connection, None)
        .await
        .expect("Migration error");

    let app = Router::new().route("/new", post(create_user));

    axum::Server::bind(&"0.0.0.0:7878".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
