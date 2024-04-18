use std::env;
use std::time;

mod utils;

use crate::utils::*;

/// Default update interval is set to 1000 millis
/// or could be set via first argument
///
/// # Examples
///
/// 1.5 seconds update interval
/// ```
/// cargo run -- 1500
/// ```
fn main() {
    let dur = if let Some(dur) = env::args().nth(1) {
        dur.parse().unwrap_or(1000)
    } else {
        1000
    };

    loop {
        cls();
        print_output_data();
        sleep(time::Duration::from_millis(dur));
    }
}
