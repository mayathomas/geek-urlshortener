use axum::{
    debug_handler,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use nanoid::nanoid;

use crate::{
    error::AppError,
    model::{ShortenReq, ShortenRes, UrlRecord},
    AppState,
};

#[debug_handler]
pub async fn url_shortener(
    State(state): State<AppState>,
    Json(req): Json<ShortenReq>,
) -> Result<impl IntoResponse, AppError> {
    let id = nanoid!(10);
    let id = UrlRecord::shorten(id, req.url, state.pool).await?;
    let res = ShortenRes {
        url: format!(
            "{}{}/{}",
            Into::<String>::into(*state.protocol),
            state.addr,
            id
        ),
    };
    Ok(Json(res))
}

pub async fn redirect(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let url = UrlRecord::get_by_id(id, state.pool).await?;

    let mut header = HeaderMap::new();
    header.insert("Location", url.parse().unwrap());
    Ok((StatusCode::FOUND, header))
}
