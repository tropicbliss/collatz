use structopt::StructOpt;

/// Collatz conjecture tester
#[derive(StructOpt, Debug)]
#[structopt()]
pub struct Args {
    /// Lower range
    #[structopt(short, long)]
    pub low: u128,

    /// Upper range
    #[structopt(short, long)]
    pub high: u128,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::from_args()
    }
}
