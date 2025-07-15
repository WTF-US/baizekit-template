pub use baizekit_api::prelude::Page;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::error::Result;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct {{entity}}DTO {
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, IntoParams)]
pub struct QueryCommand {
}

#[derive(Debug, Deserialize, Serialize, IntoParams)]
#[serde(default)]
pub struct SearchCommand {
    pub page: Option<u64>,
    pub size: Option<u64>,
}

impl Default for SearchCommand {
    fn default() -> Self {
        Self { page: Some(1), size: Some(10) }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct AddCommand {
}

#[async_trait::async_trait]
pub trait InstrumentService {
    async fn query(&self, cmd: QueryCommand) -> Result<{{entity}}DTO>;
    async fn search(&self, cmd: SearchCommand) -> Result<Page<{{entity}}DTO>>;

    async fn add(&self, cmd: AddCommand) -> Result<{{entity}}DTO>;
}
