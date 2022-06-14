mod account;
mod hashtag;
mod media;
mod publish;

pub mod prelude {
    pub use crate::graph::instagram::{account::*, hashtag::*, media::*, publish::*};
}
