use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Serialize, Deserialize, EnumIter, Display, Eq, PartialEq, Debug)]
pub enum RecommendationType {
    SimilarGenres,
    SimilarCompanies,
    Keywords,
    Recommendations,
}

impl RecommendationType {
    pub fn pt_br_title(&self) -> String {
        match self {
            RecommendationType::SimilarGenres => "Gêneros Similares".to_string(),
            RecommendationType::SimilarCompanies => "Produtoras Similares".to_string(),
            RecommendationType::Keywords => "Palavras-chave".to_string(),
            RecommendationType::Recommendations => "Recomendações".to_string(),
        }
    }
}
