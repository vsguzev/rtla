# RTLA
## Overview

RTLA (Real-Time Log Analyzer) is a powerful, efficient, and easy-to-use tool designed for real-time monitoring, analysis, and visualization of log data. It is built using Rust, offering unparalleled performance and security. RTLA is ideal for developers and system administrators who need to keep track of their systems' health and performance in real-time.

## Key Features

* Real-Time Monitoring: Stream logs from various sources and view them in real time.
* Advanced Analytics: Analyze logs with a customizable dashboard to gain insights.
* Alerts & Notifications: Set up custom rules for alerts to stay ahead of potential issues.
* Cross-Platform Support: Works seamlessly on Linux, macOS, and Windows.

# Getting Started

## Prerequisites

* Rust (latest stable version)
* Cargo (Rust's package manager)

## Building from Source

1. Clone the repository:
```bash
git clone https://github.com/vsguzev/rtla.git
```
1. Change directory to rtla:
```bash
cd rtla
```
1. Build the project using Cargo:
```bash
cargo build --release
```

## Running RTLA

To start RTLA, run:
```bash
cargo run <config_path>
```

For example, run:
```bash
cargo run configs/config.toml
```

## Usage Example

1. Configuring Log Sources: Edit config.toml to add your log sources.
1. Viewing Logs: Open the RTLA dashboard to view real-time logs.
1. Setting Alerts: Define alert rules in alerts.toml.

# Contributing

Contributions are welcome! Please read our CONTRIBUTING.md for details on our code of conduct, and the process for submitting pull requests.

# License

This project is licensed under the MIT License - see the LICENSE.md file for details.

# Support

For support, email wcbash@icloud.com or open an issue in the GitHub repository.
