use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::utils::parse_tags;

#[derive(Debug, Clone, FromRow)]
pub struct SnippetRecord {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub language: String,
    pub code: String,
    pub tags: String,
    pub is_favorite: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct SnippetResponse {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub language: String,
    pub code: String,
    pub tags: Vec<String>,
    pub is_favorite: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<SnippetRecord> for SnippetResponse {
    fn from(record: SnippetRecord) -> Self {
        Self {
            id: record.id,
            title: record.title,
            description: record.description,
            language: record.language,
            code: record.code,
            tags: parse_tags(&record.tags),
            is_favorite: record.is_favorite,
            created_at: record.created_at,
            updated_at: record.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateSnippet {
    pub title: String,

    #[serde(default)]
    pub description: String,

    pub language: String,

    pub code: String,

    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(default)]
    pub is_favorite: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSnippet {
    pub title: String,

    #[serde(default)]
    pub description: String,

    pub language: String,

    pub code: String,

    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(default)]
    pub is_favorite: bool,
}

#[derive(Debug, Deserialize)]
pub struct FavoriteRequest {
    pub is_favorite: bool,
}

#[derive(Debug, Deserialize)]
pub struct SnippetQuery {
    pub search: Option<String>,
    pub language: Option<String>,
    pub favorite: Option<bool>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct LanguageStatistic {
    pub language: String,
    pub total: i64,
}
