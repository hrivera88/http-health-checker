use anyhow::Result;
use clap::Parser;
use colored::*;
use http_health_checker::{HealthCheck, HealthChecker};
use std::time::Duration;
use tokio::time;

// A simple HTTP checker ya'll!~
#[derive(Parser)]
#[command(name = "http-health-checker")]
#[command(about = "A concurrent HTTP health checker written in Rust")]
struct Cli {
    // URLs to check (can specify multiple)
    #[arg(short, long, value_delimiter = ',')]
    urls: Vec<String>,

    //Interval between checks in seconds
    #[arg(short, long, default_value_t = 30)]
    interval: u64,

    // Output results to JSON file
    #[arg(short, long)]
    output: Option<String>,

    // Run only once (don't loop)
    #[arg(long)]
    once: bool,
}

fn print_results(results: &[HealthCheck]) {
    println!("\n{}", "Health Check Results".bold().underline());
    println!("{}", "=".repeat(60));

    for result in results {
        let status_color = match result.status.as_str() {
            "UP" => "green",
            "DOWN" => "red",
            _ => "yellow",
        };

        println!(
            "{} {} [{} ms] - {}",
            result.status.color(status_color).bold(),
            result.url.cyan(),
            result.response_time_ms,
            result.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        );

        if let Some(error) = &result.error {
            println!(" Error: {}", error.red());
        }
        if let Some(code) = result.status_code {
            println!(" Status Code: {}", code);
        }

        println!();
    }
}

async fn save_results_to_file(results: &[HealthCheck], filename: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(results)?;
    tokio::fs::write(filename, json).await?;
    println!("Results saved to {}", filename.green());
    Ok(())
}

fn get_default_urls() -> Vec<String> {
    vec![
        "https://httpbin.org/status/200".to_string(),
        "https://google.com".to_string(),
        "https://github.com".to_string(),
    ]
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let urls = if cli.urls.is_empty() {
        println!(
            "{}",
            "No URLs provided, using default test URLs ...".yellow()
        );
        get_default_urls()
    } else {
        cli.urls
    };

    let checker = HealthChecker::new(Duration::from_secs(10));
    let interval = Duration::from_secs(cli.interval);

    println!("{}", "HTTP Health Checker Starting...".green().bold());
    println!(
        "Checking {} URLs every {} seconds",
        urls.len(),
        cli.interval
    );
    println!("URLs: {}", urls.join(", ").cyan());

    if cli.once {
        println!("\n{}", "Running single check...".yellow());
    } else {
        println!("\n{}", "Press Ctrl+C to stop".yellow());
    }

    loop {
        let results = checker.check_all_urls(&urls).await;
        print_results(&results);

        if let Some(output_file) = &cli.output {
            if let Err(e) = save_results_to_file(&results, output_file).await {
                eprintln!("Failed to save results: {}", e.to_string().red());
            }
        }

        if cli.once {
            break;
        }

        time::sleep(interval).await;
    }

    Ok(())
}
