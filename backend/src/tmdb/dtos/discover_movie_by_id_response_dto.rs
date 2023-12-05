use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DiscoverMovieByIdRecomendationsMovieResponseDto {
    pub movie_id: i64,
    pub poster_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct DiscoverMovieByIdRecomendationsResponseDto {
    pub recomendation_title: String,
    pub recomendation_movies: Vec<DiscoverMovieByIdRecomendationsMovieResponseDto>,
}

#[derive(Serialize, Deserialize)]
pub struct DiscoverMovieByIdResponseDto {
    pub movie: DiscoverMovieByIdRecomendationsMovieResponseDto,
    pub recomendations: Vec<DiscoverMovieByIdRecomendationsResponseDto>,
}
