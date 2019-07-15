use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct ImageId(pub i32);

impl ImageId {
    pub fn inner(self) -> i32 {
        self.0
    }
}
