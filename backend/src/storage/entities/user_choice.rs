use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::tmdb::dtos::recommendation_type::RecommendationType;

#[derive(Serialize, Deserialize)]
pub struct UserChoice {
    pub user_uuid: Uuid,
    pub recommendation_type: RecommendationType,
    pub inserted_at: NaiveDateTime,
}
