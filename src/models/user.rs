use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub full_name: String,
    pub username: String,
    pub password: String,
    pub bio: String,
    pub profile_image: String,
}
