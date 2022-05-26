use dynomite::Attribute;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Attribute, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub enum PatronStatus {
    #[serde(alias = "active_patron")]
    Active,
    #[serde(alias = "declined_patron")]
    Declined,
    #[serde(alias = "former_patron")]
    Former,
}

impl FromStr for PatronStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(PatronStatus::Active),
            "active_patron" => Ok(PatronStatus::Active),
            "Former" => Ok(PatronStatus::Former),
            "former_patron" => Ok(PatronStatus::Former),
            "declined_patron" => Ok(PatronStatus::Declined),
            "Declined" => Ok(PatronStatus::Declined),
            _ => Err(()),
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
