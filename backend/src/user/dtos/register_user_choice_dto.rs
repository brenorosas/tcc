use crate::tmdb::dtos::recommendation_type::RecommendationType;

#[derive(serde::Deserialize)]
pub struct RegisterUserChoiceDto {
    pub recommendation_type: RecommendationType,
}
