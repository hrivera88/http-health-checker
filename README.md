# HTTP Health Checker

A concurrent HTTP health checker written in Rust that monitors the availability and response times of web services.

## Features

- 🚀 **Concurrent checking** - Check multiple URLs simultaneously using Tokio async runtime
- ⏱️ **Configurable intervals** - Set custom check intervals and timeouts
- 🎨 **Colored output** - Easy-to-read terminal output with status indicators
- 📊 **JSON export** - Save results to JSON files for analysis
- 🔧 **CLI interface** - Simple command-line interface with sensible defaults
- ⚡ **Fast and lightweight** - Minimal resource usage with Rust's zero-cost abstractions

## Installation

Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).

```bash
git clone https://github.com/yourusername/http-health-checker.git
cd http-health-checker
cargo build --release
```

## Usage

### Basic usage with default URLs
```bash
cargo run
```

### Check specific URLs
```bash
cargo run -- --urls https://google.com,https://github.com,https://example.com
```

### Run a single check (don't loop)
```bash
cargo run -- --urls https://example.com --once
```

### Custom interval and timeout
```bash
cargo run -- --urls https://example.com --interval 60 --timeout 5
```

### Save results to JSON file
```bash
cargo run -- --urls https://example.com --output results.json
```

### Combined example
```bash
cargo run -- --urls https://google.com,https://github.com --interval 30 --timeout 10 --output health_report.json
```

## Command Line Options

- `--urls, -u`: Comma-separated list of URLs to check
- `--interval, -i`: Interval between checks in seconds (default: 30)
- `--timeout, -t`: Timeout for each request in seconds (default: 10)
- `--output, -o`: Output results to JSON file
- `--once`: Run only once instead of continuous monitoring

## Example Output

```
HTTP Health Checker Starting...
Checking 3 URLs every 30 seconds
URLs: https://google.com, https://github.com, https://httpbin.org/status/200

Health Check Results
============================================================

UP https://google.com [245 ms] - 2024-01-15 14:30:45 UTC
  Status Code: 200

UP https://github.com [156 ms] - 2024-01-15 14:30:45 UTC
  Status Code: 200

DOWN https://httpbin.org/status/500 [89 ms] - 2024-01-15 14:30:45 UTC
  Error: HTTP 500
  Status Code: 500
```

## JSON Output Format

```json
[
  {
    "url": "https://google.com",
    "status": "UP",
    "status_code": 200,
    "response_time_ms": 245,
    "timestamp": "2024-01-15T14:30:45.123Z",
    "error": null
  },
  {
    "url": "https://example.com",
    "status": "DOWN",
    "status_code": null,
    "response_time_ms": 5000,
    "timestamp": "2024-01-15T14:30:45.123Z",
    "error": "timeout"
  }
]
```

## Architecture

This project demonstrates several important Rust concepts:

- **Async programming** with Tokio
- **Error handling** with the `anyhow` crate
- **Serialization** with Serde
- **CLI parsing** with Clap
- **HTTP client** with Reqwest
- **Concurrent programming** with futures
- **Structured logging** and colored output

## Dependencies

- `tokio` - Async runtime
- `reqwest` - HTTP client
- `serde` - Serialization framework
- `clap` - Command line argument parsing
- `colored` - Terminal colors
- `chrono` - Date and time handling
- `anyhow` - Error handling
- `futures` - Additional async utilities

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Future Enhancements

- [ ] Add support for custom HTTP headers
- [ ] Implement basic authentication
- [ ] Add email/Slack notifications for failures
- [ ] Support for checking TCP ports
- [ ] Web dashboard for monitoring results
- [ ] Docker containerization
- [ ] Prometheus metrics export