use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;

use action::PayloadAction;

pub mod action;
pub mod error;
#[cfg(test)]
mod test;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(bound(serialize = "T: Serialize", deserialize = "T: DeserializeOwned"))]
pub struct Payload<T>
where
    T: Serialize + DeserializeOwned + Send + Sync + Debug + PartialEq + Eq,
{
    action: PayloadAction,
    data: T,
}

impl<T> Payload<T>
where
    T: Serialize + DeserializeOwned + Send + Sync + Debug + PartialEq + Eq,
{
    pub const fn new(action: PayloadAction, data: T) -> Self {
        Payload { action, data }
    }

    pub fn from_json(json: serde_json::Value) -> Result<Self, error::PayloadError> {
        serde_json::from_value(json).map_err(|e| {
            error::PayloadError::DeserializationError(error::DeserializationError::JsonError(e))
        })
    }

    pub fn from_json_string(json: &str) -> Result<Self, error::PayloadError> {
        serde_json::from_str(json).map_err(|e| {
            error::PayloadError::DeserializationError(error::DeserializationError::JsonError(e))
        })
    }

    pub fn to_json(&self) -> Result<serde_json::Value, error::PayloadError> {
        serde_json::to_value(self).map_err(|e| {
            error::PayloadError::SerializationError(error::SerializationError::JsonError(e))
        })
    }

    pub fn to_json_string(&self) -> Result<String, error::PayloadError> {
        serde_json::to_string(self).map_err(|e| {
            error::PayloadError::SerializationError(error::SerializationError::JsonError(e))
        })
    }

    pub fn to_json_string_pretty(&self) -> Result<String, error::PayloadError> {
        serde_json::to_string_pretty(self).map_err(|e| {
            error::PayloadError::SerializationError(error::SerializationError::JsonError(e))
        })
    }
}

impl<T> Payload<T>
where
    T: Serialize + DeserializeOwned + Send + Sync + Debug + PartialEq + Eq,
{
    pub fn action(&self) -> &PayloadAction {
        &self.action
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}
