use std::sync::Arc;

use axum::{debug_handler, extract::Query, Extension, Json};

use crate::{
    controller::errors::ErrorResponse,
    tmdb::{
        dtos::{
            discover_movie_dto::DiscoverMovieDto,
            discover_movie_response_dto::DiscoverMovieResponseDto,
        },
        service::TmdbService,
    },
};

#[debug_handler]
pub async fn tmdb_movie_discover_handler(
    Extension(tmdb_service): Extension<Arc<TmdbService>>,
    Query(params): Query<DiscoverMovieDto>,
) -> Result<Json<DiscoverMovieResponseDto>, ErrorResponse> {
    let mut response = tmdb_service
        .discover_movie(params)
        .await
        .map_err(|error| ErrorResponse::from(error))?;

    response
        .results
        .retain(|result| result.poster_path.is_some());

    for result in response.results.iter_mut() {
        result.poster_path = Some(format!(
            "https://image.tmdb.org/t/p/w500{}",
            result.poster_path.clone().unwrap()
        ));
    }

    Ok(Json(response))
}
