use std::sync::Arc;

use axum::{debug_handler, extract::Path, Extension, Json};

use crate::{
    controller::errors::ErrorResponse,
    tmdb::{
        dtos::discover_movie_by_id_response_dto::DiscoverMovieByIdResponseDto, service::TmdbService,
    },
};

#[debug_handler]
pub async fn tmdb_movie_discover_by_id_handler(
    Extension(tmdb_service): Extension<Arc<TmdbService>>,
    Path(movie_id): Path<i64>,
) -> Result<Json<DiscoverMovieByIdResponseDto>, ErrorResponse> {
    let response = tmdb_service
        .discover_movie_by_id(movie_id)
        .await
        .map_err(|error| ErrorResponse::from(error))?;

    Ok(Json(response))
}
