use serde::{Serialize, Deserialize};

use crate::models::ImageId;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum Response {
    Error(String),
    Ids(Vec<ImageId>)
}
