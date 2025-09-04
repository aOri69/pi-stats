//! # Crate lib

mod cli;
mod error;
mod platform;

pub use cli::App;
pub use error::Error;
pub use platform::Rpi;

pub type Result<T> = std::result::Result<T, Error>;
