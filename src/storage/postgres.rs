use log::info;
use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use num_traits::FromPrimitive;
use failure::{Error, format_err};

use crate::models::{ImageId, Image, ImageFormat, LoadedImage};
use super::Storage;

type Pool<C> = r2d2::Pool<ConnectionManager<C>>;

pub struct PostgresStorage {
    connections: Pool<PgConnection>
}

impl PostgresStorage {
    pub fn new(url: &str) -> Result<Self, Error> {
        let manager = ConnectionManager::new(url);
        let pool = r2d2::Pool::builder().build(manager)?;
        
        Ok(PostgresStorage {
            connections: pool
        })
    }
}

impl Storage for PostgresStorage {
    fn store(&self, imgs: Vec<Image>) -> Result<Vec<ImageId>, Error> { 
        use crate::schema::images::dsl::*;
        
        let connection = self.connections.get()?;
        let imgs = imgs.into_iter()
            .map(|image| image.insertable())
            .collect::<Result<Vec<_>, Error>>()?;
        
        let ids: Vec<ImageId> = diesel::insert_into(images)
            .values(&imgs)
            .returning(image_id)
            .get_results::<i32>(&connection)?
            .into_iter()
            .map(ImageId)
            .collect();
       
        let total_size: usize = imgs.iter().map(|img| img.data.len()).sum();
        info!("Successfully stored {} images (total size is {}): {:?}", 
              ids.len(), total_size, ids);
        Ok(ids)
    }

    fn load(&self, id: ImageId) -> Result<Image, Error> {
        use crate::schema::images::dsl::*;

        let connection = self.connections.get()?;
        let image = images.filter(image_id.eq(id.inner()))
            .load::<LoadedImage>(&connection)?;

        let image = image.into_iter()
            .next()
            .ok_or_else(|| format_err!("No image with id={}", id.inner()))?;
        
        let f = ImageFormat::from_i32(image.format)
            .ok_or_else(|| format_err!("Unknown image format: {}", image.format))?;

        Image::decode(&image.data, f)
    }
}
