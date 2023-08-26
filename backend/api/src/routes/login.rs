use axum::{extract::Json, routing::post};
use serde::Deserialize;
use service::prelude::*;

use crate::state::{ApiRouter, ApiState};

pub fn routes() -> ApiRouter {
    ApiRouter::new().route("/password", post(with_password))
}

#[derive(Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

async fn with_password(
    api: ApiState,
    Json(cred): Json<Credentials>,
) -> Result<Json<String>, SvcErr> {
    let user = user::validate_password(
        cred.username,
        cred.password,
        api.config.pepper.clone(),
        api.config.pepper_old.clone(),
        &api.db,
    )
    .await?;

    let token = session::create(&user, &api.config.session_secret, &api.db).await?;
    Ok(Json(token))
}
