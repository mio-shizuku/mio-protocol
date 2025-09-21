use serde::Serialize;

mod error;
#[cfg(test)]
mod test;

#[derive(Serialize)]
pub struct Payload<T: Serialize + Send + Sync> {
    data: T,
}

impl<T: Serialize + Send + Sync> Payload<T> {
    pub const fn new(data: T) -> Self {
        Payload { data }
    }

    pub fn to_json(&self) -> Result<serde_json::Value, error::PayloadError> {
        serde_json::to_value(self).map_err(|e| {
            error::PayloadError::SerializationError(error::SerializationError::JsonError(e))
        })
    }
}
