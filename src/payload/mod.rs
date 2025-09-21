use serde::{Serialize, de::DeserializeOwned};

mod error;
#[cfg(test)]
mod test;

#[derive(Serialize)]
#[serde(bound(serialize = "T: Serialize"))]
pub struct Payload<T: Serialize + DeserializeOwned + Send + Sync> {
    data: T,
}

impl<T: Serialize + DeserializeOwned + Send + Sync> Payload<T> {
    pub const fn new(data: T) -> Self {
        Payload { data }
    }

    pub fn to_json(&self) -> Result<serde_json::Value, error::PayloadError> {
        serde_json::to_value(self).map_err(|e| {
            error::PayloadError::SerializationError(error::SerializationError::JsonError(e))
        })
    }
}
