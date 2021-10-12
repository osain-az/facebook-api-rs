pub mod accounts;
pub mod client;
pub mod data;
pub mod image;
pub mod instagram;
pub mod me;
pub mod pages;
pub mod utils;
pub mod prelude {
    pub use crate::graph::{
        accounts::*, client::*, data::*, image::*, instagram::*, me::*, pages::*, utils::*,
    };
}
