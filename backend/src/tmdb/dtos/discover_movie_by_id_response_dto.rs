use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DiscoverMovieByIdRecommendationsMovieResponseDto {
    pub movie_id: i64,
    pub poster_path: String,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct DiscoverMovieByIdRecommendationsResponseDto {
    pub recommendation_title: String,
    pub recommendation_movies: Vec<DiscoverMovieByIdRecommendationsMovieResponseDto>,
}

#[derive(Serialize, Deserialize)]
pub struct DiscoverMovieByIdResponseDto {
    pub movie: DiscoverMovieByIdRecommendationsMovieResponseDto,
    pub recommendations: Vec<DiscoverMovieByIdRecommendationsResponseDto>,
}
