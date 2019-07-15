use std::sync::RwLock;

use failure::{Error, format_err};

use super::Storage;
use crate::models::{Image, ImageId};

#[derive(Default)]
pub struct MemoryStorage {
    table: RwLock<Vec<Image>>
}

impl MemoryStorage {
    pub fn with_image(image: Image) -> Self {
        MemoryStorage {
            table: RwLock::new(vec![image])
        }
    }
}

impl Storage for MemoryStorage {
    fn store(&self, images: Vec<Image>) -> Result<Vec<ImageId>, Error> {
        let mut table = self.table.write().unwrap();
        let start = table.len() as i32;
        table.extend(images.into_iter());
        let end = table.len() as i32;
        let ids = (start..end).into_iter()
            .map(ImageId)
            .collect();
        Ok(ids)
    }

    fn load(&self, id: ImageId) -> Result<Image, Error> {
        let table = self.table.read().unwrap();
        table.get(id.inner() as usize)
            .cloned()
            .ok_or_else(|| format_err!("Image with id={} not found", id.inner()))
    }
}
