use std::time::Instant;

use broll::{Input, Output, core::run_simulations};
use clap::Parser;

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let input: Input = config::Config::builder()
        .add_source(config::File::with_name("input").required(true))
        .build()
        .expect("Config build failed")
        .try_deserialize()
        .expect("Config deserialization failed");

    input.validate()?;

    let start = Instant::now();
    let results = run_simulations(&input, cli.seed, cli.num_simulations);
    let duration = start.elapsed();

    let output = Output::new(duration.as_secs_f64(), &input, &results);

    println!("{}", serde_json::to_string_pretty(&output).unwrap());

    Ok(())
}

#[derive(Debug, Parser)]
#[command(about = "Broll | Poker Bankroll Simulator | CLI")]
struct Cli {
    /// Optional seed for reproducibility
    #[arg(short, long)]
    seed: Option<u64>,

    /// Number of simulations
    #[arg(short, long, default_value_t = 30_000)]
    num_simulations: usize,
}
