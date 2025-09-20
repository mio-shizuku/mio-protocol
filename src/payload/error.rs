#[derive(Debug)]
pub enum PayloadError {
    SerializationError(SerializationError),
}

#[derive(Debug)]
pub enum SerializationError {
    JsonError(serde_json::Error),
}
