use clap::Parser;

const DEFAULT_EXPONENT: usize = 7;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    /// Exponent of 10 (10^?) for the number of runs to perform in total
    #[arg(default_value_t = DEFAULT_EXPONENT)]
    pub max_exponent: usize,

    /// Save csv file of game run data
    #[arg(short, long)]
    pub save_data: bool,

    /// Print less information (only show final results / show no result at all)
    ///
    /// Give once for mostly quiet, and twice for completely quiet.
    ///
    /// When printing results, only print results after all runs, or nothing.
    ///
    /// When saving data, only print that data has been saved to file, or nothing.
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub quiet: u8,

    /// Print the time elapsed during calulation
    #[arg(short, long)]
    pub elapsed_time: bool,

    /// Number of threads to spawn
    #[arg(short, long, default_value_t = 7, require_equals = true)]
    pub threads: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
