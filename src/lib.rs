//! # Crate lib

mod cli;
mod error;
mod throttle;
// mod power;
// mod commands;

pub use cli::App;
pub use error::Error;
pub use throttle::*;

pub type CpuTemp = f32;
pub type CpuClock = u32;
pub type Result<T> = std::result::Result<T, Error>;

pub struct Clock {
    pub arm: CpuClock,
    pub gpu: CpuClock,
}

pub struct PowerStatus;

pub trait Platform {
    fn get_cputemp(&self) -> Result<CpuTemp>;
    fn get_clock(&self) -> Result<CpuTemp>;
    fn get_throttle(&self) -> Result<ThrottleStatus>;
    fn get_power(&self) -> Result<PowerStatus>;
}
