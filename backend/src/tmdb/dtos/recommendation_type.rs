use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Serialize, Deserialize, EnumIter, Display, Eq, PartialEq, Debug)]
pub enum RecommendationType {
    SimilarGenres,
}

impl RecommendationType {
    pub fn pt_br_title(&self) -> String {
        match self {
            RecommendationType::SimilarGenres => "Gêneros Similares".to_string(),
        }
    }
}
