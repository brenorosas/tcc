use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DiscoverMovieResponseDto {
    pub page: i64,
    pub results: Vec<DiscoverMovieResultDto>,
    pub total_pages: i64,
    pub total_results: i64,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DiscoverMovieGenreDto {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DiscoverMovieResultDto {
    pub id: i64,
    pub title: String,
    pub overview: String,
    pub poster_path: Option<String>,
    #[serde(default)]
    pub genres: Vec<DiscoverMovieGenreDto>,
    #[serde(default)]
    pub production_companies: Vec<DiscoverMovieProductionCompanyDto>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DiscoverMovieProductionCompanyDto {
    pub id: i64,
}
