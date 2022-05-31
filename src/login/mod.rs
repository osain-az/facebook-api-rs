pub mod config;
pub mod login;
pub mod response_type;
pub mod token;
pub mod prelude {
    pub use crate::login::{config::*, login::*, response_type::*, token::*};
    pub use crate::universal::client::*;
}
