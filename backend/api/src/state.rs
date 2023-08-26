use std::sync::Arc;

use axum::{extract::State, routing::Router};
use service::sea_orm::DatabaseConnection;

use crate::config::ApiConfig;

pub type ApiRouter = Router<Arc<Api>>;
pub type ApiState = State<Arc<Api>>;

pub struct Api {
    pub config: ApiConfig,

    pub db: DatabaseConnection,
}
