use structopt::StructOpt;

/// Solver
#[derive(StructOpt, Debug)]
#[structopt()]
pub struct Args {
    /// Low number
    #[structopt(short, long)]
    pub low: u128,

    /// High number
    #[structopt(short, long)]
    pub high: u128,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::from_args()
    }
}
