use serde::{Deserialize, Serialize};
///Struct for which can hold data types.
#[derive(Deserialize, Debug, Serialize)]
pub struct Data<T> {
    pub data: Vec<T>,
}
