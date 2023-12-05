pub mod errors;

use crate::utils::env::get_var;
use std::time::Duration;

use self::errors::TmdbServiceError;

use super::dtos::{
    discover_movie_by_id_response_dto::{
        DiscoverMovieByIdRecomendationsMovieResponseDto,
        DiscoverMovieByIdRecomendationsResponseDto, DiscoverMovieByIdResponseDto,
    },
    discover_movie_dto::DiscoverMovieDto,
    discover_movie_response_dto::DiscoverMovieResponseDto,
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
            .get(format!("{}/discover/movie", self.base_url))
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
        Ok(DiscoverMovieByIdResponseDto {
            movie: DiscoverMovieByIdRecomendationsMovieResponseDto {
                movie_id: movie_id + 20,
                poster_path: "breno".to_owned(),
            },
            recomendations: vec![DiscoverMovieByIdRecomendationsResponseDto {
                recomendation_title: "Generos parecidos".to_owned(),
                recomendation_movies: vec![
                    DiscoverMovieByIdRecomendationsMovieResponseDto {
                        movie_id: movie_id + 21,
                        poster_path: "levi".to_owned(),
                    },
                    DiscoverMovieByIdRecomendationsMovieResponseDto {
                        movie_id: movie_id + 22,
                        poster_path: "thalita".to_owned(),
                    },
                ],
            }],
        })
    }
}
