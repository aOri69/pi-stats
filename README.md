# Raspberry Pi System Monitor

A **command-line tool** to monitor and log Raspberry Pi system parameters—CPU temperature, ARM clock speed, throttling status, power consumption, and fan data—in real time.

## Features

- **Live output** of system data: temperature, ARM clock (MHz), throttling status, total power (W), fan PWM, and fan RPM
- **Customizable update interval** (default: 1000 ms; override with command-line argument)
- **Graceful Ctrl+C handling** for a clean shutdown
- **CSV format output**, suitable for direct logging

## Usage

```shell
cargo run -- <INTERVAL_MS>
```

- `<INTERVAL_MS>` is an optional update interval in milliseconds (default: `1000`).

**Example:**

```shell
cargo run -- 1500
```

## Piping Output to CSV

This tool is intended to be used with a shell pipe to save output directly to a CSV file.

**Write new log (overwrite):**

```shell
cargo run -- 1000 > monitor_data.csv
```

**Append to existing log:**

```shell
cargo run -- 1000 >> monitor_data.csv
```

**Add timestamps (first column) using `awk`:**

```shell
cargo run -- 1000 | awk -v OFS=',' '{ print strftime("%Y-%m-%d %H:%M:%S"), $0 }' > monitor_data.csv
```

**Log only a fixed number of samples (e.g., 10):**

```shell
cargo run -- 1000 | head -n 10 > monitor_data.csv
```

## Output Format

Each line is a CSV row:

`<Temp>,<Clock MHz>,<Throttle>,<Power(W)>,<PWM>,<Fan RPM>`

Where:
- **Temp**: CPU temperature (°C)
- **Clock MHz**: ARM frequency (MHz)
- **Throttle**: `true` if currently throttled
- **Power(W)**: Total estimated power (W)
- **PWM**: Fan PWM duty
- **Fan RPM**: Fan speed (RPM)

## Example Output

`48.9,1500,true,2.513,80,2400`


## Ctrl+C Handling

Pressing **Ctrl+C** will print a message and exit gracefully.

## Requirements

- **Rust** toolchain (for compilation and running)
- `vcgencmd` utility (included on Raspberry Pi OS)
- Fan monitoring files at `/sys/devices/platform/cooling_fan/hwmon/*/{pwm1,fan1_input}` (if hardware supports it)

## How It Works

- The tool reads command-line arguments for the update interval (in milliseconds).
- On each loop:
  - Clears the console
  - Gathers sensor and system data
  - Prints a formatted CSV line to stdout
  - Sleeps for the specified interval
- A custom Ctrl+C handler allows for interruption and exit at any time.

## License

MIT
