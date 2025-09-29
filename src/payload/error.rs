#[derive(Debug)]
pub enum PayloadError {
    SerializationError(SerializationError),
    DeserializationError(DeserializationError),
}

impl std::fmt::Display for PayloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PayloadError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            PayloadError::DeserializationError(e) => write!(f, "Deserialization error: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum SerializationError {
    JsonError(serde_json::Error),
}

impl std::fmt::Display for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerializationError::JsonError(e) => write!(f, "JSON serialization error: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum DeserializationError {
    JsonError(serde_json::Error),
}

impl std::fmt::Display for DeserializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeserializationError::JsonError(e) => write!(f, "JSON deserialization error: {}", e),
        }
    }
}
