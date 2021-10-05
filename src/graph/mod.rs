pub mod accounts;
pub mod client;
pub mod data;
pub mod get_posts;
pub mod image;
pub mod instagram;
pub mod me;
pub mod pages;
pub mod post;
pub mod utils;
pub mod video;
pub mod prelude {
    pub use crate::graph::{
        accounts::*, client::*, data::*, get_posts::*, image::*, instagram::*, me::*, pages::*,
        post::*, utils::*, video::*,
    };
}
