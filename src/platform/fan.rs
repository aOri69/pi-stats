use std::fs;

use crate::Result;

pub type PwmValue = u8;
pub type RpmValue = u32;

const FAN_PATH: &str = "/sys/devices/platform/cooling_fan/hwmon";
const FAN_PWM_SUFFIX: &str = "pwm1";
const FAN_RPS_SUFFIX: &str = "fan1_input";

#[derive(Default)]
pub struct FanStatus {
    pub pwm: PwmValue,
    pub rpm: RpmValue,
}

impl FanStatus {
    pub fn new() -> Result<Self> {
        //FIXME Need to find more elegant solution to this
        //FIXME Error propogation
        let mut hwmon_dir = fs::read_dir(FAN_PATH)?
            .next()
            .expect("expected one folder under hwmon")?
            .path();

        if !hwmon_dir.is_dir() {
            return Err(crate::Error::ParseCommand(String::from("error")));
        }

        hwmon_dir.push(FAN_PWM_SUFFIX);
        let pwm = fs::read_to_string(hwmon_dir.as_path())?;
        hwmon_dir.pop();
        hwmon_dir.push(FAN_RPS_SUFFIX);
        let rpm = fs::read_to_string(hwmon_dir.as_path())?;

        Ok(Self {
            pwm: pwm.trim().parse()?,
            rpm: rpm.trim().parse()?,
        })
    }
}
