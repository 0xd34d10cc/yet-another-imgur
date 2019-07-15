use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Serialize, Deserialize};
use failure::{Error, format_err};
use enum_primitive_derive::Primitive;

#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Primitive, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageFormat {
    PNG = 1,
    JPEG = 2,
    GIF = 3,
    BMP = 4,
    ICO = 5
}

impl ImageFormat {
    pub fn from_content_type(content_type: &str) -> Result<ImageFormat, Error> {
        match content_type {
            "image/png" => Ok(ImageFormat::PNG),
            "image/jpeg" => Ok(ImageFormat::JPEG),
            "image/gif" => Ok(ImageFormat::GIF),
            "image/bmp" => Ok(ImageFormat::BMP),
            "image/ico" => Ok(ImageFormat::ICO),
            other => Err(format_err!("Unknown image format: {}", other))
        }
    }
}

impl Display for ImageFormat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ImageFormat::PNG => "png",
            ImageFormat::JPEG => "jpeg",
            ImageFormat::GIF => "gif",
            ImageFormat::BMP => "bmp",
            ImageFormat::ICO => "ico"
        };

        write!(formatter, "{}", s)
    }
}

impl FromStr for ImageFormat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "png" => Ok(ImageFormat::PNG),
            "jpeg" => Ok(ImageFormat::JPEG),
            "gif" => Ok(ImageFormat::GIF),
            "bmp" => Ok(ImageFormat::BMP),
            "ico" => Ok(ImageFormat::ICO),
            other => Err(format_err!("Unknown image format: {}", other))
        }
    }
}

impl Into<image::ImageFormat> for ImageFormat {
    fn into(self) -> image::ImageFormat {
        match self {
            ImageFormat::PNG => image::ImageFormat::PNG,
            ImageFormat::JPEG => image::ImageFormat::JPEG,
            ImageFormat::GIF => image::ImageFormat::GIF,
            ImageFormat::BMP => image::ImageFormat::BMP,
            ImageFormat::ICO => image::ImageFormat::ICO
        }
    }
}

impl Into<image::ImageOutputFormat> for ImageFormat {
    fn into(self) -> image::ImageOutputFormat {
        match self {
            ImageFormat::PNG => image::ImageOutputFormat::PNG,
            ImageFormat::JPEG => image::ImageOutputFormat::JPEG(75 /* quality */),
            ImageFormat::GIF => image::ImageOutputFormat::GIF,
            ImageFormat::BMP => image::ImageOutputFormat::BMP,
            ImageFormat::ICO => image::ImageOutputFormat::ICO
        }
    }
}
