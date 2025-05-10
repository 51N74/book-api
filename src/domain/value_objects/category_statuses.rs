use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CategoryStatuses {
    #[default]
    Open,
    Borrowed,
}

impl Display for CategoryStatuses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CategoryStatuses::Open => write!(f, "Open"),
            CategoryStatuses::Borrowed => write!(f, "Borrowed"),
        }
    }
}