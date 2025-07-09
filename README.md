# ws-ping

`ws-ping` is a command-line utility written in Rust for pinging WebSocket servers and measuring their latency. It functions similarly to the traditional `ping` command but is tailored for WebSocket connections, sending WebSocket Ping frames and awaiting Pong responses.

## Features

- Ping any WebSocket URL (`ws://` or `wss://`).
- Specify the number of pings to send.
- Set the interval between pings.
- Detailed output including Round Trip Time (RTT), minimum, maximum, average latency, and mean deviation.

## Installation

To install `ws-ping`, you need to have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your system.

```bash
cargo install ws-ping
```

## Usage

```bash
ws-ping <URL> [OPTIONS]
```

### Arguments

- `<URL>`: The WebSocket URL to ping (e.g., `ws://echo.websocket.events` or `wss://stream.binance.com:9443/ws/btcusdt@depth`).

### Options

- `-c`, `--count <COUNT>`: Number of pings to send. Defaults to `4`.
- `-i`, `--interval <INTERVAL>`: Interval between pings in seconds. Defaults to `1`.

## Examples

1. **Basic ping to a WebSocket echo server:**

   ```bash
   ws-ping ws://echo.websocket.events
   ```

2. **Ping a secure WebSocket with 10 pings and a 0.5-second interval:**

   ```bash
   ws-ping wss://stream.binance.com:9443/ws/btcusdt@depth -c 10 -i 0.5
   ```

3. **Ping with default options:**

   ```bash
   ws-ping wss://your-websocket-server.com/ws
   ```

## Repository

The source code for `ws-ping` is available on GitHub: [https://github.com/maggnus/websocket-ping](https://github.com/maggnus/websocket-ping)

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
