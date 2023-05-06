use ::entity::prelude::User;
use ::entity::user::{self, Model};
use from_body::UserSerialized::UserSerialized;
use sea_orm::*;
use uuid::Uuid;

pub async fn create_new_user(user_data: UserSerialized, db: &DatabaseConnection) {}

pub async fn find_all_user(db: DatabaseConnection) -> Vec<Model> {
    User::find().all(&db).await.unwrap()
}

pub async fn find_user_by_id(id: Uuid, db: DatabaseConnection) -> Option<Model> {
    User::find_by_id(id)
        .one(&db)
        .await
        .expect("User not found.")
}
