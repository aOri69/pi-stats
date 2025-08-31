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

pub struct InnerThrottleStatus {
    pub under_voltage: bool,
    pub arm_frequency_capped: bool,
    pub throttled: bool,
    pub soft_temp_limit: bool,
}

pub struct ThrottleStatus {
    pub current: InnerThrottleStatus,
    pub happened: InnerThrottleStatus,
}

impl std::str::FromStr for ThrottleStatus {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
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

#[inline]
fn bit_is_set(input: u32, bit: u32) -> bool {
    (input & (1u32 << bit)) != 0
}
