pub mod accounts;
pub mod client;
pub mod data;
pub mod get_posts;
pub mod image;
pub mod me;
pub mod pages;
pub mod post;
pub  mod video;
pub mod prelude {
    pub use crate::graph::{
        accounts::*, client::*, data::*, get_posts::*, image::*, me::*, pages::*, post::*,video::*,
    };
}
