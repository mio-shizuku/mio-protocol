use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum PayloadAction {
    #[serde(rename = "auth")]
    Auth,
    #[serde(rename = "test")]
    Test,
}
