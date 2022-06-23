use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub struct Engagement {
    /// Number of people who like this
    count: u32,
}

impl Engagement {
    pub fn count(&self) -> u32 {
        self.count
    }
}

pub enum MediaType {
    Video,
    Image,
}
