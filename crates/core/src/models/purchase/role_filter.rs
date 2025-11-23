use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum OrderRoleFilter {
    /// The user created the order
    Creator,
    /// The user is the receiver of the order
    Receiver,
    /// The user owns at least one item in the order
    Participant,
}
