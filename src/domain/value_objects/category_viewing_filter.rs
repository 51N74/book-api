use serde::{Deserialize, Serialize};

use super::category_statuses::CategoryStatuses;



#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CategoryViewingFilter {
    pub name: Option<String>,
    pub status: Option<CategoryStatuses>,
}