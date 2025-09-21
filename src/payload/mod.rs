use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::fmt::Debug;

mod error;
#[cfg(test)]
mod test;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(bound(serialize = "T: Serialize", deserialize = "T: DeserializeOwned"))]
pub struct Payload<T>
where
    T: Serialize + DeserializeOwned + Send + Sync + Debug + PartialEq + Eq,
{
    data: T,
}

impl<T> Payload<T>
where
    T: Serialize + DeserializeOwned + Send + Sync + Debug + PartialEq + Eq,
{
    pub const fn new(data: T) -> Self {
        Payload { data }
    }

    pub fn from_json(json: serde_json::Value) -> Result<Self, error::PayloadError> {
        serde_json::from_value(json).map_err(|e| {
            error::PayloadError::DeserializationError(error::DeserializationError::JsonError(e))
        })
    }

    pub fn to_json(&self) -> Result<serde_json::Value, error::PayloadError> {
        serde_json::to_value(self).map_err(|e| {
            error::PayloadError::SerializationError(error::SerializationError::JsonError(e))
        })
    }
}
