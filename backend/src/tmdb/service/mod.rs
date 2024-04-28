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
    recommendation_type::RecommendationType,
};
use reqwest::{Client, ClientBuilder};
use strum::IntoEnumIterator;

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

    pub async fn get_movies_with_genres(
        &self,
        genres: Vec<i64>,
    ) -> Result<DiscoverMovieResponseDto, TmdbServiceError> {
        let with_genres = genres
            .iter()
            .map(|genre| genre.to_string())
            .collect::<Vec<String>>()
            .join("|");

        let response = self
            .client
            .get(format!(
                "{}/discover/movie?with_genres={}&language=pt-BR",
                self.base_url, with_genres
            ))
            .bearer_auth(&self.bearer_token)
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

    pub async fn get_movies_with_companies(
        &self,
        companies: Vec<i64>,
    ) -> Result<DiscoverMovieResponseDto, TmdbServiceError> {
        let with_companies = companies
            .iter()
            .map(|company| company.to_string())
            .collect::<Vec<String>>()
            .join("|");

        let response = self
            .client
            .get(format!(
                "{}/discover/movie?with_companies={}&language=pt-BR",
                self.base_url, with_companies
            ))
            .bearer_auth(&self.bearer_token)
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

    pub async fn get_movies_by_similar_keywords(
        &self,
        main_movie_id: i64,
    ) -> Result<DiscoverMovieResponseDto, TmdbServiceError> {
        let response = self
            .client
            .get(format!(
                "{}/movie/{}/similar?language=pt-BR",
                self.base_url, main_movie_id
            ))
            .bearer_auth(&self.bearer_token)
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

    pub async fn get_movies_by_recommendations(
        &self,
        main_movie_id: i64,
    ) -> Result<DiscoverMovieResponseDto, TmdbServiceError> {
        let response = self
            .client
            .get(format!(
                "{}/movie/{}/recommendations?language=pt-BR",
                self.base_url, main_movie_id
            ))
            .bearer_auth(&self.bearer_token)
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
            .get(format!(
                "{}/movie/{}?language=pt-BR",
                self.base_url, movie_id
            ))
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

        main_movie.poster_path = Some(format!(
            "https://image.tmdb.org/t/p/w500{}",
            main_movie.poster_path.unwrap()
        ));

        let mut recommendations = vec![];
        for recommendation_type in RecommendationType::iter() {
            let movies = match recommendation_type {
                RecommendationType::SimilarGenres => {
                    self.get_movies_with_genres(
                        main_movie.genres.iter().map(|genre| genre.id).collect(),
                    )
                    .await?
                }
                RecommendationType::SimilarCompanies => {
                    self.get_movies_with_companies(
                        main_movie
                            .production_companies
                            .iter()
                            .map(|company| company.id)
                            .collect(),
                    )
                    .await?
                }
                RecommendationType::Keywords => {
                    self.get_movies_by_similar_keywords(main_movie.id).await?
                }
                RecommendationType::Recommendations => {
                    self.get_movies_by_recommendations(main_movie.id).await?
                }
            };
            recommendations.push(DiscoverMovieByIdRecommendationsResponseDto {
                recommendation_title: recommendation_type.pt_br_title(),
                recommendation_movies: movies.results,
                recommendation_type,
            });
        }

        for recommendation in recommendations.iter_mut() {
            recommendation
                .recommendation_movies
                .retain(|movie| movie.id != main_movie.id && movie.poster_path.is_some());
        }

        for recommendation in recommendations.iter_mut() {
            for movie in recommendation.recommendation_movies.iter_mut() {
                movie.poster_path = Some(format!(
                    "https://image.tmdb.org/t/p/w500{}",
                    movie.poster_path.clone().unwrap()
                ));
            }
        }

        Ok(DiscoverMovieByIdResponseDto {
            movie: main_movie,
            recommendations,
        })
    }
}
