use dynomite::Attribute;
use serde::{Deserialize, Serialize};

#[derive(Attribute, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub enum Role {
    User,
    Admin,
}
