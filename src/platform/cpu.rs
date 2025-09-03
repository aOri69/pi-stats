use std::ops::Deref;

use crate::{
    command::{Arg, Vcgencmd},
    Error, Result,
};

pub type ClockValue = u32;
pub type TempValue = f32;

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
    pub arm: ClockValue,
    pub gpu: ClockValue,
}

impl CpuClock {
    pub fn new() -> Result<Self> {
        Ok(Self {
            arm: Vcgencmd::run(&[Arg::MeasureClockArm.as_str()])?
                .trim()
                .strip_prefix("frequency(0)=")
                .ok_or(Error::ParseCommand(String::from(
                    "Failed to strip prefix: frequency(0)=",
                )))?
                .parse()?,
            gpu: Vcgencmd::run(&[Arg::MeasureClockCore.as_str()])?
                .trim()
                .strip_prefix("frequency(0)=")
                .ok_or(Error::ParseCommand(String::from(
                    "Failed to strip prefix: frequency(0)=",
                )))?
                .parse()?,
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
