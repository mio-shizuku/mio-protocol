use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Pin {
    code: String,
}

impl Pin {
    pub fn new(code: String) -> Self {
        Pin { code }
    }

    pub fn code(&self) -> &str {
        &self.code
    }
}

impl From<String> for Pin {
    fn from(code: String) -> Self {
        Pin::new(code)
    }
}

impl From<&str> for Pin {
    fn from(code: &str) -> Self {
        Pin::new(code.to_string())
    }
}

impl From<Pin> for String {
    fn from(val: Pin) -> Self {
        val.code
    }
}
