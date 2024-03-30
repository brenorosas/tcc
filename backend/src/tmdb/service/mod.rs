pub mod errors;

use crate::utils::env::get_var;
use std::time::Duration;

use self::errors::TmdbServiceError;

use super::dtos::{
    discover_movie_by_id_response_dto::{
        DiscoverMovieByIdRecommendationsResponseDto, DiscoverMovieByIdResponseDto,
    },
    discover_movie_dto::DiscoverMovieDto,
    discover_movie_response_dto::{DiscoverMovieResponseDto, DiscoverMovieResultDto},
};
use reqwest::{Client, ClientBuilder};

pub struct TmdbService {
    base_url: String,
    bearer_token: String,
    client: Client,
}

impl TmdbService {
    pub fn new() -> Self {
        let bearer_token =
            get_var("TMDB_BEARER_TOKEN").unwrap_or_else(|| "tmdb_bearer_token".to_string());
        let base_url =
            get_var("TMDB_BASE_URL").unwrap_or_else(|| "https://api.themoviedb.org/3".to_string());
        let client = ClientBuilder::new()
            .pool_idle_timeout(Some(Duration::from_secs(60)))
            .tcp_keepalive(Some(Duration::from_secs(60)))
            .timeout(Duration::from_secs(40))
            .build()
            .expect("should be able to build http client");

        Self {
            base_url,
            bearer_token,
            client,
        }
    }

    pub async fn discover_movie(
        &self,
        dto: DiscoverMovieDto,
    ) -> Result<DiscoverMovieResponseDto, TmdbServiceError> {
        let response = self
            .client
            .get(format!("{}/discover/movie?language=pt-BR", self.base_url))
            .bearer_auth(&self.bearer_token)
            .query(&dto)
            .send()
            .await;

        match response {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => response
                    .json::<DiscoverMovieResponseDto>()
                    .await
                    .map_err(|_| TmdbServiceError::UnexpectedResponseDto),
                reqwest::StatusCode::UNAUTHORIZED => {
                    return Err(TmdbServiceError::InvalidApiKey);
                }
                status => {
                    return Err(TmdbServiceError::UnexpectedTmdbStatusCode(status));
                }
            },
            Err(error) => Err(TmdbServiceError::Unknown(error.into())),
        }
    }
    pub async fn discover_movie_by_id(
        &self,
        movie_id: i64,
    ) -> Result<DiscoverMovieByIdResponseDto, TmdbServiceError> {
        let response = self
            .client
            .get(format!("{}/movie/{}?language=pt-BR", self.base_url, movie_id))
            .bearer_auth(&self.bearer_token)
            .send()
            .await;

        let mut main_movie = match response {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => response
                    .json::<DiscoverMovieResultDto>()
                    .await
                    .map_err(|_| TmdbServiceError::UnexpectedResponseDto),
                reqwest::StatusCode::UNAUTHORIZED => {
                    return Err(TmdbServiceError::InvalidApiKey);
                }
                status => {
                    return Err(TmdbServiceError::UnexpectedTmdbStatusCode(status));
                }
            },
            Err(error) => Err(TmdbServiceError::Unknown(error.into())),
        }?;

        main_movie.poster_path =
            format!("https://image.tmdb.org/t/p/w500{}", main_movie.poster_path);

        let with_genres = main_movie
            .genres
            .iter()
            .map(|genre| genre.id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let mut movies_with_same_genre = self
            .client
            .get(format!(
                "{}/discover/movie?with_genres={}&language=pt-BR",
                self.base_url, with_genres
            ))
            .bearer_auth(&self.bearer_token)
            .send()
            .await
            .map_err(|error| TmdbServiceError::Unknown(error.into()))?
            .json::<DiscoverMovieResponseDto>()
            .await
            .map_err(|_| TmdbServiceError::UnexpectedResponseDto)?;

        for result in movies_with_same_genre.results.iter_mut() {
            result.poster_path = format!("https://image.tmdb.org/t/p/w500{}", result.poster_path);
        }
        
        movies_with_same_genre.results.retain(|movie| movie.id != main_movie.id);

        Ok(DiscoverMovieByIdResponseDto {
            movie: main_movie,
            recommendations: vec![
                DiscoverMovieByIdRecommendationsResponseDto {
                    recommendation_title: "Gêneros parecidos".to_owned(),
                    recommendation_movies: movies_with_same_genre.results.clone(),
                },
                DiscoverMovieByIdRecommendationsResponseDto {
                    recommendation_title: "Gêneros parecidos".to_owned(),
                    recommendation_movies: movies_with_same_genre.results,
                },
            ],
        })
    }
}
