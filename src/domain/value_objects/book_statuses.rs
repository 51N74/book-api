use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BookStatuses {
    #[default]
    Open,
    Borrowed,
}

impl Display for BookStatuses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BookStatuses::Open => write!(f, "Open"),
            BookStatuses::Borrowed => write!(f, "Borrowed"),
        }
    }
}