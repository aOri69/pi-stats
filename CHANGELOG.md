# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Semantic Versioning](https://semver.org/).

## [0.2.0] - 2025-09-11

### Added
- Migrated the TUI to the new Ratatui library for better UI features.
- Added async tokio runtime. Main loop is now async.
- Introduced build script for strict target: `aarch64-unknown-linux-gnu`.

### Changed
- Complete code refactor for improved readability and maintainability.
- Separated TUI logic from main data getters `Platform` and `App`.

### Fixed
- Fixed unusual `Ctrl+C` handling behavior.

### Removed
- Removed support for non-Linux and non-ARM platforms (e.g., Windows, x86).

## [0.1.0] - 2024-04-23

### Added
- Initial implementation of CPU temperature monitoring.
- Initial implementation of Throttle byte parsing based on `vcgencmd`.
- Initial implementation of Power Consumption metrics.
- Initial implementation of Fan status - RPM and PWM.
- Console output of temperature data in text format.

### Fixed
- Minor fixes in temperature parsing logic.
