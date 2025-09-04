use std::process;

use crate::Result;

const VCGENCMD: &str = "vcgencmd";

pub enum Arg {
    MeasureTemp,  // measure_temp
    MeasureClock, // measure_clock
    Arm,          // arm
    Core,         // core
    PmicReadAdc,  // pmic_read_adc
    GetThrottled, // get_throttled
}

impl Arg {
    pub fn as_str(&self) -> &str {
        match self {
            Arg::MeasureTemp => "measure_temp",
            Arg::MeasureClock => "measure_clock",
            Arg::Arm => "arm",
            Arg::Core => "core",
            Arg::PmicReadAdc => "pmic_read_adc",
            Arg::GetThrottled => "get_throttled",
        }
    }
}

pub struct Vcgencmd;

impl Vcgencmd {
    pub fn run(args: &[&str]) -> Result<String> {
        Ok(String::from_utf8(
            process::Command::new(VCGENCMD).args(args).output()?.stdout,
        )?)
    }
}
