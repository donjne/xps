use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::users::register::Register;

#[derive(Deserialize)]
struct BlinksRequest {
    username: String,
}

#[derive(Serialize, Deserialize)]
struct BlinksResponse {
    icon: String,
    title: String,
    description: String,
    label: String,
    disabled: Option<bool>,
    links: Option<Links>,
    error: Option<ActionError>,
}

#[derive(Serialize, Deserialize)]
struct Links {
    actions: Vec<LinkedAction>,
}

#[derive(Serialize, Deserialize)]
struct LinkedAction {
    href: String,
    label: String,
    parameters: Option<Vec<ActionParameter>>,
}

#[derive(Serialize, Deserialize)]
struct ActionParameter {
    name: String,
    label: Option<String>,
    required: Option<bool>,
}

#[derive(Serialize, Deserialize)]
struct ActionError {
    // Define the fields of ActionError as needed
    // Example:
    code: Option<String>,
    message: Option<String>,
}
async fn blinks_action(
    db: web::Data<Arc<Mutex<Surreal>>>,
    form: web::Json<BlinksRequest>,
) -> impl Responder {
    let mut db = db.lock().await;
    let user = db.query("SELECT * FROM user WHERE username = $username")
        .bind(("username", &form.username))
        .await.unwrap()
        .take::<Vec<Register>>(0)
        .unwrap()
        .pop()
        .unwrap();

    HttpResponse::Ok().json(BlinksResponse {
        icon: user.profile_image,
        title: format!("Donate to {}", user.username),
        description: user.bio,
        label: "Donate".to_string(),
    })
}
