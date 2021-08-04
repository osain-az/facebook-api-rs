use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Me {
    name: String,
    user_id: String,
}
