pub const DB: DatabaseConnection = Database::connect(
    "postgres://postgres:toujours_skateboarding@localhost:5432/toujours_skateboarding_axum_db",
)
.await?;
