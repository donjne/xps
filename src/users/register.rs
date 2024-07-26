use actix_web::{web, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::Mutex;
use sha2::{Sha512, Digest};

use crate::error::CustomError;

#[derive(Deserialize, Serialize)]
pub struct Register {
    id: Option<String>,
    email: String,
    full_name: String,
    username: String,
    password: String,
    bio: String,
    profile_image: String,
}

async fn register(
    db: web::Data<Arc<Mutex<Surreal<surrealdb::sql::Db>>>>,
    form: web::Json<Register>,
) -> impl Responder {

    let new_user = form.into_inner();

    if !is_email_valid(&new_user.email) {
        return HttpResponse::BadRequest().body("Invalid email format");
    }

    let hashed_password = match hash_password(&new_user.password) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to hash password"),
    };

    let user_id = Uuid::new_v4().to_string();
    let user = Register {
        id: Some(user_id),
        email: new_user.email,
        full_name: new_user.full_name,
        username: new_user.username,
        password: hashed_password,
        bio: new_user.bio,
        profile_image: new_user.profile_image,
    };

    let mut db = db.lock().await;
    match db.query(format!(
        "CREATE user SET id = '{}', email = '{}', full_name = '{}', username = '{}', password = '{}', bio = '{}', profile_image = '{}'",
        user.id.as_ref().unwrap(),
        user.email,
        user.full_name,
        user.username,
        user.password,
        user.bio,
        user.profile_image
    )).await {
        Ok(_) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create user"),
    }
}

fn is_email_valid(email: &str) -> bool {
    email.contains('@')
}

fn hash_password(password: &str) -> Result<String, CustomError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt)
        .map(|password_hash| password_hash.to_string())
        .map_err(|e| CustomError::from(e))
}

pub fn verify_password(hash: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    let parsed_hash = PasswordHash::new(hash)?;
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)?;
    Ok(())
}
