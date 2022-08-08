pub mod accounts;
mod batch;
pub mod client;
pub mod data;
pub mod image;
pub mod instagram;
pub mod me;
pub mod pages;
pub mod utils;

pub mod prelude {
    pub use crate::graph::{
        accounts::*, batch::request::*, client::*, data::*, image::*, instagram::prelude::*, me::*,
        pages::*, utils::*,
    };
}
