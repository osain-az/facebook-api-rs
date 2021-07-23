use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Data<T>
{
    pub data : T
}
