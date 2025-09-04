use std::{ops::Deref, str::FromStr};

use crate::{
    platform::command::{Arg, Vcgencmd},
    Error, Result,
};

pub type ClockHz = u32;
pub type ClockMhz = f32;
pub type TempValue = f32;

#[derive(Default)]
pub struct Clock(ClockMhz);

impl Deref for Clock {
    type Target = ClockMhz;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Clock {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Clock((s.parse::<ClockHz>()? as ClockMhz) / 1_000_000.0))
    }
}

#[derive(Default)]
pub struct CpuTemp(TempValue);

impl Deref for CpuTemp {
    type Target = TempValue;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CpuTemp {
    pub fn new() -> Result<Self> {
        Ok(Self(
            Vcgencmd::run(&[Arg::MeasureTemp.as_str()])?
                .trim()
                .strip_prefix("temp=")
                .ok_or(Error::ParseCommand(String::from(
                    "Failed to strip prefix: temp=",
                )))?
                .strip_suffix("'C")
                .ok_or(Error::ParseCommand(String::from(
                    "Failed to strip suffix: 'C",
                )))?
                .parse()?,
        ))
    }
}

#[derive(Default)]
pub struct CpuClock {
    pub arm: Clock,
    pub gpu: Clock,
}

impl CpuClock {
    pub fn new() -> Result<Self> {
        use Arg::*;
        Ok(Self {
            arm: Vcgencmd::run(&[MeasureClock.as_str(), Arm.as_str()])?
                .trim()
                .strip_prefix("frequency(0)=")
                .ok_or(Error::ParseCommand(String::from(
                    "Failed to strip prefix: frequency(0)=",
                )))?
                .parse::<Clock>()?,
            gpu: Vcgencmd::run(&[MeasureClock.as_str(), Core.as_str()])?
                .trim()
                .strip_prefix("frequency(0)=")
                .ok_or(Error::ParseCommand(String::from(
                    "Failed to strip prefix: frequency(0)=",
                )))?
                .parse::<Clock>()?,
        })
    }
}

#[derive(Default)]
pub struct CpuStatus {
    pub clock: CpuClock,
    pub temp: CpuTemp,
}

impl CpuStatus {
    pub fn new() -> Result<Self> {
        Ok(Self {
            clock: CpuClock::new()?,
            temp: CpuTemp::new()?,
        })
    }
}
