use std::sync::Arc;

use axum::Router;
use sea_orm::DatabaseConnection;

use self::state::AppState;
use crate::_db::*;
use crate::service::*;

pub mod http;
pub mod state;

pub async fn setup(db: Arc<DatabaseConnection>) -> (AppState, Router, Router) {
    todo!()
}
