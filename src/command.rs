use std::process;

use crate::Result;

const VCGENCMD: &str = "vcgencmd";

pub enum Arg {
    MeasureTemp,      // measure_temp
    MeasureClockArm,  // measure_clock arm
    MeasureClockCore, // measure_clock core
    PmicReadAdc,      // pmic_read_adc
    GetThrottled,     // get_throttled
}

impl Arg {
    pub fn as_str(&self) -> &str {
        match self {
            Arg::MeasureTemp => "measure_temp",
            Arg::MeasureClockArm => "measure_clock arm",
            Arg::MeasureClockCore => "measure_clock core",
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
