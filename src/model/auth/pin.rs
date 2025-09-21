use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Pin {
    code: String,
}

impl Pin {
    pub fn new(code: String) -> Self {
        Pin { code }
    }
}
