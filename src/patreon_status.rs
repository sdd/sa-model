use std::fmt;
use dynomite::Attribute;
use serde::{Deserialize, Serialize};


#[derive(Attribute, Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[non_exhaustive]
pub enum PatronStatus {
    #[serde(alias = "active_patron")]
    Active,
    #[serde(alias = "declined_patron")]
    Declined,
    #[serde(alias = "former_patron")]
    Former,
}

impl PatronStatus {
    pub fn from_str(role: &str) -> PatronStatus {
        match role {
            "Active" => PatronStatus::Active,
            "active_patron" => PatronStatus::Active,
            "Former" => PatronStatus::Former,
            "former_patron" => PatronStatus::Former,
            "declined_patron" => PatronStatus::Declined,
            _ => PatronStatus::Declined,
        }
    }
}

impl fmt::Display for PatronStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatronStatus::Active => write!(f, "Active"),
            PatronStatus::Declined => write!(f, "Declined"),
            PatronStatus::Former => write!(f, "Former"),
        }
    }
}
