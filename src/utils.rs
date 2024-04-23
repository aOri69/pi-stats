use std::{collections::HashMap, process, thread, time};

const VCGEN: &str = "vcgencmd";

enum Arg {
    MeasureTemp,     // measure_temp
    MeasureClockArm, // measure_clock arm
    PmicReadAdc,     // pmic_read_adc
    GetThrottled,    // get_throttled
}

impl Arg {
    fn as_str(&self) -> &str {
        match self {
            Arg::MeasureTemp => "measure_temp",
            Arg::MeasureClockArm => "measure_clock arm",
            Arg::PmicReadAdc => "pmic_read_adc",
            Arg::GetThrottled => "get_throttled",
        }
    }
}

pub fn cls() {
    // process::Command::new("clear").status().unwrap();
    process::Command::new("clear")
        .spawn()
        .expect("clear command failed to start")
        .wait()
        .expect("failed to wait");
}

pub fn sleep(dur: time::Duration) {
    thread::sleep(dur);
}

pub fn print_output_data() {
    let (pwm, rpm) = get_fan_data().expect("expected to get fan data");

    println!(
        // "{:<5},{:<6},{:<7}.{:<8.3}\n",
        "{:},{:},{:},{:.3},{:},{:}",
        get_temp(),
        get_clock(),
        get_throttle(),
        get_power_consumption().iter().sum::<f32>(),
        pwm,
        rpm
    );
}

fn run_command_as_utf(cmd_name: &str, args: &[&str]) -> String {
    String::from_utf8(
        process::Command::new(cmd_name)
            .args(args)
            .output()
            .expect("failed to run vcgencmd")
            .stdout,
    )
    .expect("error converting to string")
}

fn get_fan_data() -> std::io::Result<(u32, u32)> {
    use std::fs;

    const FAN_PATH: &str = "/sys/devices/platform/cooling_fan/hwmon";
    const FAN_PWM_SUFFIX: &str = "pwm1";
    const FAN_RPS_SUFFIX: &str = "fan1_input";

    let mut hwmon_dir = fs::read_dir(FAN_PATH)?
        .next()
        .expect("expected one folder under hwmon")?
        .path();

    if hwmon_dir.is_dir() {
        hwmon_dir.push(FAN_PWM_SUFFIX);
        let pwm = fs::read_to_string(hwmon_dir.as_path())?;
        hwmon_dir.pop();
        hwmon_dir.push(FAN_RPS_SUFFIX);
        let rps = fs::read_to_string(hwmon_dir.as_path())?;
        return Ok((
            pwm.trim().parse().expect("expected valid pwm duty number"),
            rps.trim().parse().expect("expected valid RPS number"),
        ));
    }

    Ok((0, 0))
}

fn hex_as_byte_string(hex_string: &str) -> String {
    // // Remove the "0x" prefix if present
    let hex_string = match hex_string.strip_prefix("0x") {
        Some(stripped) => stripped,
        None => hex_string,
    };

    // Hexadecimal value
    let hex_value = i32::from_str_radix(hex_string, 16).unwrap();
    // Convert hexadecimal value to binary string
    let binary_string = format!("{:020b}", hex_value);
    binary_string
}

/// Function is breaking down the hexadecimal output
/// of the `vcgencmd get_throttled` output
///
/// ## Bit representation pattern
///
/// | Bit | Meaning                             |
/// | :-: | ----------------------------------- |
/// |  0  | Under-voltage detected              |
/// |  1  | Arm frequency capped                |
/// |  2  | Currently throttled                 |
/// |  3  | Soft temperature limit active       |
/// | 16  | Under-voltage has occurred          |
/// | 17  | Arm frequency capped has occurred   |
/// | 18  | Throttling has occurred             |
/// | 19  | Soft temperature limit has occurred |
///
fn get_throttle() -> bool {
    let output = run_command_as_utf(VCGEN, &[Arg::GetThrottled.as_str()]);
    let output = output.split('=').nth(1).unwrap().trim();
    let binary_string = hex_as_byte_string(output);
    // dbg!(&output, &binary_string);

    let mut breakdown = HashMap::new();

    let mut rev_it = binary_string[16..].chars().rev();
    breakdown.insert("0", rev_it.next().unwrap().to_digit(10).unwrap());
    breakdown.insert("1", rev_it.next().unwrap().to_digit(10).unwrap());
    breakdown.insert("2", rev_it.next().unwrap().to_digit(10).unwrap());
    breakdown.insert("3", rev_it.next().unwrap().to_digit(10).unwrap());

    let mut rev_it = binary_string[..4].chars().rev();
    breakdown.insert("16", rev_it.next().unwrap().to_digit(10).unwrap());
    breakdown.insert("17", rev_it.next().unwrap().to_digit(10).unwrap());
    breakdown.insert("18", rev_it.next().unwrap().to_digit(10).unwrap());
    breakdown.insert("19", rev_it.next().unwrap().to_digit(10).unwrap());

    match breakdown.get("2").unwrap() {
        0 => false,
        1 => true,
        _ => unreachable!(),
    }
}

fn get_temp() -> f32 {
    let output = run_command_as_utf(VCGEN, &[Arg::MeasureTemp.as_str()]);
    output
        .trim()
        .split('=')
        .nth(1)
        .unwrap()
        .split('\'')
        .next()
        .unwrap()
        .parse()
        .unwrap()
}

fn get_clock() -> u32 {
    let output = run_command_as_utf(VCGEN, &[Arg::MeasureClockArm.as_str()]);
    let (_str, frq) = output.trim().split_once('=').unwrap();
    // frq.parse::<usize>().unwrap() / 1000000
    frq[..4].parse().unwrap()
}

fn get_power_consumption() -> Vec<f32> {
    #[derive(Debug)]
    struct Measurement {
        volts: f32,
        amps: f32,
    }

    let output = run_command_as_utf(VCGEN, &[Arg::PmicReadAdc.as_str()]);
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

        let entry = measurements.entry(key).or_insert(Measurement {
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

    measurements.values().map(|m| m.volts * m.amps).collect()
}
