use std::env;
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
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

    let stop_flag = Arc::new(Mutex::new(false));
    let ctrlc_rx = spawn_ctrlc_handler();

    loop {
        cls();

        if ctrlc_rx.try_recv().is_ok() {
            println!("\nCtrl+C recieved\nGraceful shutdown...");
            *stop_flag.lock().unwrap() = true;
            break;
        }

        print_output_data();

        sleep(time::Duration::from_millis(dur));
    }
}

fn spawn_ctrlc_handler() -> Receiver<()> {
    let (ctrlc_tx, ctrlc_rx) = channel();
    ctrlc::set_handler(move || {
        ctrlc_tx
            .send(())
            .expect("Could not send signal on channel.")
    })
    .expect("Error setting Ctrl-C handler");

    ctrlc_rx
}
