use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserSerialized {
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub cellnumber: String,
    pub mail: String,
    pub password: String,
    pub secretkey: String,
}
