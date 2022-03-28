use serde::{Deserialize, Serialize};
use dynomite::Attribute;

#[derive(Attribute, Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[non_exhaustive]
pub enum Role {
    User,
    Admin,
}
