pub mod config;
pub mod redirect_url;
pub mod response_type;
pub mod token;
pub mod prelude {
    pub use crate::login::{config::*, redirect_url::*, response_type::*, token::*};
    pub use crate::universal::client::*;
}
