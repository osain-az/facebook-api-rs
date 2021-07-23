use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Image {
    height:u16,
    width:u16,
    is_silhouette:bool,
    pub url:String,
}
