use axum::Json;
use axum::extract::{Path, Query, State};
use {{project-name | snake_case}}_sdk::service::{{entity}}::*;
use {{project-name | snake_case}}_sdk::error::ApiResult;
use {{project-name | snake_case}}_sdk::prelude::{ApiError, ApiOK, Page, Reply};
use uuid::Uuid;

use crate::setup::state::AppState;

#[utoipa::path(
    get,
    path = "/{{entity}}s",
    summary = "搜索",
    tags = ["{{entity | pascal_case}}"],
    params(SearchCommand),
    responses(
        (status = 200, description = "Success", body = Reply<Page<{{entity | pascal_case}}DTO>>),
    ),
)]
pub async fn get_{{entity}}s(
    State(state): State<AppState>,
    Query(cmd): Query<SearchCommand>,
) -> ApiResult<Page<{{entity | pascal_case}}DTO>> {
    state.{{entity}}_svc.search(cmd).await.map(ApiOK::with_data).map_err(ApiError)
}

#[utoipa::path(
    post,
    path = "/{{entity}}s",
    summary = "添加",
    tags = ["{{entity | pascal_case}}"],
    request_body(content = inline(AddCommand)),
    responses(
        (status = 200, description = "Success", body = Reply<{{entity | pascal_case}}DTO>),
    ),
)]
pub async fn post_{{entity}}s(State(state): State<AppState>, Json(cmd): Json<AddCommand>) -> ApiResult<{{entity | pascal_case}}DTO> {
    state.{{entity}}_svc.add(cmd).await.map(ApiOK::with_data).map_err(ApiError)
}

#[utoipa::path(
    put,
    path = "/{{entity}}/{id}",
    summary = "更新",
    tags = ["{{entity | pascal_case}}"],
    params(
        ("id" = Uuid, description = "账号ID"),
    ),
    request_body(content = inline(UpdateCommand)),
    responses(
        (status = 200, description = "Success", body = Reply<{{entity | pascal_case}}DTO>),
    ),
)]
pub async fn put_{{entity}}(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(mut cmd): Json<UpdateCommand>,
) -> ApiResult<{{entity | pascal_case}}DTO> {
    cmd.id = id;
    state.{{entity}}_svc.update(cmd).await.map(ApiOK::with_data).map_err(ApiError)
}
