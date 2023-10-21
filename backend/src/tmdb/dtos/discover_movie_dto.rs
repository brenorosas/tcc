use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DiscoverMovieDto {
    pub page: i64,
}
