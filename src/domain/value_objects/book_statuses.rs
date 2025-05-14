use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BookStatuses {
    #[default]
    Available,
    Reserved,
}

impl Display for BookStatuses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BookStatuses::Available => write!(f, "Available"),
            BookStatuses::Reserved => write!(f, "Borrowed"),
        }
    }
}