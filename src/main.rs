#![warn(clippy::pedantic)]

mod cli;
mod logic;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let args = cli::Args::parse_args();
    if args.low == args.high {
        logic::solve_single_number(args.low).with_context(|| "Error solving number.")?;
    } else {
        logic::prove_numbers(args.low, args.high).with_context(|| "Error proving numbers.")?;
    }
    Ok(())
}
