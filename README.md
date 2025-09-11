# Raspberry Pi System Monitor`

A **command-line tool** to monitor and log Raspberry Pi system parameters—CPU temperature, ARM clock speed, throttling status, power consumption, and fan data—in real time.

> **Disclaimer**
> This project is developed primarily for learning the Rust programming language and for personal use.

## Features

- Live output presented in a terminal UI with power consumption chart
- Dynamic update interval adjustment inside the UI with + and - keys
- Power consumption chart
- Clean exit with q, Esc, or Ctrl+C
- tiny resource requirements and async main loop

## Installation

Can be installed via cargo:

```shell
cargo install --path .
```

## Usage

Simply run the program without arguments:

```shell
cargo run
```

### User Controls

- `q` or `Esc` — quit the application
- `Ctrl+C` — quit gracefully
- `+` — increase update interval
- `-` — decrease update interval

## Output

Information is displayed live in the terminal. 
A scrolling chart shows recent power consumption data.

## Planned Features

- CSV or log file output for recorded data
- historical chart feature based on the recorded data

## Requirements

- **Rust** toolchain (for compilation and running)
- `vcgencmd` utility (included on Raspberry Pi OS)
- Fan monitoring files at `/sys/devices/platform/cooling_fan/hwmon/*/{pwm1,fan1_input}` (if hardware supports it - it definitely should)

## License

[MIT](LICENSE)