pub use baizekit_api::prelude::Page;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::error::Result;

#[derive(Debug, Serialize, ToSchema)]
pub struct {{entity | pascal_case}}DTO {
    /// ID
    pub id: Uuid,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct QueryCommand {
    pub id: Uuid,
}

#[derive(Debug, Deserialize, IntoParams)]
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

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddCommand {
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateCommand {
    #[serde(skip)]
    pub id: Uuid,
}

#[async_trait::async_trait]
pub trait {{entity | pascal_case}}Service: Send + Sync {
    async fn query(&self, cmd: QueryCommand) -> Result<{{entity | pascal_case}}DTO>;

    async fn search(&self, cmd: SearchCommand) -> Result<Page<{{entity | pascal_case}}DTO>>;

    async fn add(&self, cmd: AddCommand) -> Result<{{entity | pascal_case}}DTO>;

    async fn update(&self, cmd: UpdateCommand) -> Result<{{entity | pascal_case}}DTO>;
}
