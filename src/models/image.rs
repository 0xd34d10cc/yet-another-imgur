use failure::Error;

use super::format::ImageFormat;
use crate::schema::images;

#[derive(Queryable)]
pub struct LoadedImage {
    pub id: i32,
    pub format: i32,
    pub data: Vec<u8>
}

#[derive(Insertable)]
#[table_name="images"]
pub struct NewImage {
    pub format: i32,
    pub data: Vec<u8>
}

#[derive(Clone)]
pub struct Image {
    format: ImageFormat,
    data: image::DynamicImage
}

impl Image {
    pub fn decode(data: &[u8], format: ImageFormat) -> Result<Image, Error> {
        let data = image::load_from_memory_with_format(data, format.into())?;
        
        Ok(Image {
            format,
            data
        })
    }
    
    pub fn data(&self) -> Result<Vec<u8>, Error> {
        let mut data = Vec::new();
        let format: image::ImageOutputFormat = self.format.into();
        self.data.write_to(&mut data, format)?;
        Ok(data)
    }

    pub fn format(&self) -> ImageFormat {
        self.format
    }

    pub fn insertable(&self) -> Result<NewImage, Error> {
        Ok(NewImage {
            format: self.format as i32,
            data: self.data()?
        })
    }

    pub fn preview(&self, (width, height): (u32, u32)) -> Image {
        Image {
            format: self.format,
            data: self.data.resize(width, height, image::FilterType::Lanczos3)
        }
    }
}

