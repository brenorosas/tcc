use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DiscoverMovieResponseDto {
    pub page: i64,
    pub results: Vec<DiscoverMovieResultDto>,
    pub total_pages: i64,
    pub total_results: i64,
}

#[derive(Deserialize, Serialize)]
pub struct DiscoverMovieResultDto {
    pub id: i64,
    pub title: String,
    pub overview: String,
    pub poster_path: String,
    pub backdrop_path: Option<String>,
}
