use failure::Error;
use crate::models::{ImageId, Image};

#[cfg(test)]
mod memory;
mod postgres;

#[cfg(test)]
pub use self::memory::MemoryStorage;
pub use self::postgres::PostgresStorage;

pub trait Storage: 'static + Send + Sync {
    fn store(&self, images: Vec<Image>) -> Result<Vec<ImageId>, Error>;
    fn load(&self, id: ImageId) -> Result<Image, Error>;
}
