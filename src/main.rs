use clap::Parser;
use futures_util::{SinkExt, StreamExt};
use rustls::crypto::ring;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tokio_tungstenite;
use tungstenite::Message;
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// WebSocket URL to ping
    url: String,

    /// Number of pings to send
    #[arg(short, long, default_value_t = 4)]
    count: u64,

    /// Interval between pings in seconds
    #[arg(short, long, default_value_t = 1)]
    interval: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let url = Url::parse(&args.url)?;
    let host = url.host_str().unwrap_or("unknown").to_string();

    let resolved_ips: Vec<std::net::SocketAddr> = tokio::net::lookup_host(format!(
        "{}:{}",
        host,
        url.port_or_known_default().unwrap_or(80)
    ))
    .await?
    .collect();
    let ip_address = resolved_ips
        .get(0)
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let scheme = url.scheme();

    println!(
        "PING {} ({}) with {} pings, {}s interval...",
        args.url, ip_address, args.count, args.interval
    );

    let total_start_time = Instant::now();

    let (mut ws_stream, _) = tokio_tungstenite::connect_async(&args.url).await?;

    let mut rtts = Vec::new();
    let mut packets_sent = 0;
    let mut packets_received = 0;

    for i in 0..args.count {
        packets_sent += 1;
        let start_time = Instant::now();

        // Send a ping message
        ws_stream.send(Message::Ping(vec![].into())).await?;

        // Wait for a pong response
        let mut received_pong = false;
        while let Some(msg) = ws_stream.next().await {
            match msg? {
                Message::Pong(_) => {
                    let rtt = start_time.elapsed();
                    rtts.push(rtt);
                    packets_received += 1;
                    println!(
                        "{} {}: ping_seq={} latency={:.3}ms",
                        scheme,
                        ip_address,
                        i + 1,
                        rtt.as_secs_f64() * 1000.0
                    );
                    received_pong = true;
                    break;
                }
                _ => {
                    // Ignore other messages for now
                }
            }
        }

        if !received_pong {
            println!(
                "{} to {}: ping_seq={} No pong received",
                scheme,
                ip_address,
                i + 1
            );
        }

        if i < args.count - 1 {
            sleep(Duration::from_secs(args.interval)).await;
        }
    }

    let total_time_elapsed = total_start_time.elapsed();

    println!(
        "
--- {} ping statistics ---",
        host
    );
    println!(
        "{} requests submitted, {} received, {:.2}% responses failed, time {:.0}ms",
        packets_sent,
        packets_received,
        ((packets_sent - packets_received) as f64 / packets_sent as f64) * 100.0,
        total_time_elapsed.as_secs_f64() * 1000.0
    );

    if !rtts.is_empty() {
        let min_rtt = rtts.iter().min().unwrap();
        let max_rtt = rtts.iter().max().unwrap();
        let sum_rtt: Duration = rtts.iter().sum();
        let avg_rtt = sum_rtt / rtts.len() as u32;

        // Calculate mean deviation (mdev)
        let avg_rtt_ms = avg_rtt.as_secs_f64() * 1000.0;
        let sum_of_squared_diffs: f64 = rtts
            .iter()
            .map(|&rtt| (rtt.as_secs_f64() * 1000.0 - avg_rtt_ms).powi(2))
            .sum();
        let mdev = (sum_of_squared_diffs / rtts.len() as f64).sqrt();

        println!(
            "Ping-pong latency: {:.3}ms (Min: {:.3}ms, Max: {:.3}ms, Avg: {:.4}ms, Mdev: {:.2}ms)",
            avg_rtt.as_secs_f64() * 1000.0,
            min_rtt.as_secs_f64() * 1000.0,
            max_rtt.as_secs_f64() * 1000.0,
            avg_rtt.as_secs_f64() * 1000.0,
            mdev,
        );
    }

    ws_stream.close(None).await?;

    Ok(())
}
