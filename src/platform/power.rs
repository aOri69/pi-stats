//! Struct is breaking down the hexadecimal output
//! of the `vcgencmd get_throttled` output
//!
//! ## Bit representation pattern
//!
//! | Bit | Meaning                             |
//! | :-: | ----------------------------------- |
//! |  0  | Under-voltage detected              |
//! |  1  | Arm frequency capped                |
//! |  2  | Currently throttled                 |
//! |  3  | Soft temperature limit active       |
//! | 16  | Under-voltage has occurred          |
//! | 17  | Arm frequency capped has occurred   |
//! | 18  | Throttling has occurred             |
//! | 19  | Soft temperature limit has occurred |
//!

use std::{collections::HashMap, str::FromStr};

use crate::{
    platform::command::{Arg, Vcgencmd},
    Result,
};
#[derive(Debug, Default)]
pub struct InnerThrottleStatus {
    pub under_voltage: bool,
    pub arm_frequency_capped: bool,
    pub throttled: bool,
    pub soft_temp_limit: bool,
}

#[derive(Debug, Default)]
pub struct ThrottleStatus {
    pub current: InnerThrottleStatus,
    pub happened: InnerThrottleStatus,
}

impl std::str::FromStr for ThrottleStatus {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        //FIXME from_str implementation for ThrottleStatus
        let hex = s
            .trim()
            .strip_prefix("throttled=")
            .unwrap_or(s)
            .strip_prefix("0x")
            .or(s.strip_prefix("0X"))
            .unwrap_or(s);

        let hex = u32::from_str_radix(hex, 16)?;

        Ok(ThrottleStatus {
            current: InnerThrottleStatus {
                under_voltage: bit_is_set(hex, 0),
                arm_frequency_capped: bit_is_set(hex, 1),
                throttled: bit_is_set(hex, 2),
                soft_temp_limit: bit_is_set(hex, 3),
            },
            happened: InnerThrottleStatus {
                under_voltage: bit_is_set(hex, 16),
                arm_frequency_capped: bit_is_set(hex, 17),
                throttled: bit_is_set(hex, 18),
                soft_temp_limit: bit_is_set(hex, 19),
            },
        })
    }
}

impl ThrottleStatus {
    pub fn new() -> Result<Self> {
        Ok(ThrottleStatus::from_str(&Vcgencmd::run(&[
            Arg::GetThrottled.as_str(),
        ])?)?)
    }
}

#[inline]
fn bit_is_set(input: u32, bit: u32) -> bool {
    (input & (1u32 << bit)) != 0
}

pub type Watt = f32;
pub type Volt = f32;
pub type Amp = f32;

#[derive(Debug)]
pub struct PowerMeasure {
    pub measure: String,
    pub volts: Volt,
    pub amps: Amp,
}

#[derive(Debug, Default)]
pub struct Power {
    pub power_map: Vec<PowerMeasure>,
    pub total_power: Watt,
}

impl Power {
    pub fn new() -> Result<Self> {
        //FIXME Power map
        //FIXME Error propogation
        let output = Vcgencmd::run(&[Arg::PmicReadAdc.as_str()])?;
        let mut measurements = HashMap::new();

        for line in output.lines() {
            let mut it = line.split_ascii_whitespace();
            let key = it.next().expect("expected the name of the parameter");

            let (key, value_type) = key
                .trim()
                .rsplit_once('_')
                .expect("expected the values ended with suffix _A or _V");

            let value = it
                .next()
                .expect("expected value")
                .split('=')
                .nth(1)
                .expect("expected value after the name");

            let entry = measurements.entry(key).or_insert(PowerMeasure {
                measure: key.to_owned(),
                volts: 0f32,
                amps: 0f32,
            });

            match value_type {
                "A" => {
                    entry.amps = value
                        .trim_end_matches('A')
                        .parse()
                        .expect("expected float number")
                }
                "V" => {
                    entry.volts = value
                        .trim_end_matches('V')
                        .parse()
                        .expect("expected float number")
                }
                _ => unreachable!(),
            }
        }

        let mut power_map = measurements.into_values().collect::<Vec<_>>();
        power_map.sort_unstable_by(|a, b| a.measure.cmp(&b.measure));

        let total_power: Watt = power_map
            .iter()
            .map(|measure| measure.volts * measure.amps)
            .sum();

        Ok(Self {
            power_map,
            total_power,
        })
    }
}

#[derive(Debug, Default)]
pub struct PowerStatus {
    pub throttle: ThrottleStatus,
    pub power: Power,
}

impl PowerStatus {
    pub fn new() -> Result<Self> {
        Ok(Self {
            throttle: ThrottleStatus::new()?,
            power: Power::new()?,
        })
    }
}
