use serde::{Deserialize, Serialize};
use crate::tmdb::dtos::discover_movie_response_dto::DiscoverMovieResultDto;

#[derive(Serialize, Deserialize)]
pub struct DiscoverMovieByIdRecommendationsResponseDto {
    pub recommendation_title: String,
    pub recommendation_movies: Vec<DiscoverMovieResultDto>,
}

#[derive(Serialize, Deserialize)]
pub struct DiscoverMovieByIdResponseDto {
    pub movie: DiscoverMovieResultDto,
    pub recommendations: Vec<DiscoverMovieByIdRecommendationsResponseDto>,
}
