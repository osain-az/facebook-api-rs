pub mod config;
pub mod login_parameters;
pub mod response_type;
pub mod token;
pub mod prelude {
    pub use crate::login::{config::*, login_parameters::*, response_type::*, token::*};
    pub use crate::universal::client::*;
}
