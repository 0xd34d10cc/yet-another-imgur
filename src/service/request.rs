use failure::Error;
use serde::{Serialize, Deserialize};

use crate::models::{Image, ImageFormat};


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Request {
    Base64 {
        images: Vec<Base64Image>
    },
    Remote {
        locations: Vec<String>
    }
}

#[derive(Serialize, Deserialize)]
pub struct Base64Image {
    pub format: ImageFormat,
    pub data: String
}

impl Base64Image {
    pub fn decode(&self) -> Result<Image, Error> {
        let binary_data = base64::decode(&self.data)?;
        Image::decode(&binary_data, self.format)
    }
}

