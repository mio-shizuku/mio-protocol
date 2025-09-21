#[derive(Debug)]
pub enum PayloadError {
    SerializationError(SerializationError),
    DeserializationError(DeserializationError),
}

#[derive(Debug)]
pub enum SerializationError {
    JsonError(serde_json::Error),
}

#[derive(Debug)]
pub enum DeserializationError {
    JsonError(serde_json::Error),
}
