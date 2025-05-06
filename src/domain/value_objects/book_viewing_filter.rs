use serde::{Deserialize, Serialize};

use super::book_statuses::BookStatuses;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BookViewingFilter {
    pub title: Option<String>,
    pub status: Option<BookStatuses>,
}