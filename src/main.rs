mod notes;
mod intervals;
mod interval_sequences;
mod interval_set;

use clap::Parser;
use intervals::Interval;

/// Compute just intonated interval sequences that drift in tuning
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Frequency scaling parameter
    #[arg(short, long, default_value_t = 1.0)]
    freq_scale: f64,

    /// Number of half steps
    #[arg(short, long, default_value_t = 0)]
    nhalf_steps: i32,
}

fn main() {
    // command line parsing
    let args = Args::parse();

    let mut interval_list = Vec::<Interval>::new();
    interval_list.push(Interval::major_third());
    interval_list.push(Interval::perfect_fifth());

    let mut combi_interval: Interval = Interval::perfect_fourth();
    for iinterval in interval_list {
        combi_interval = combi_interval.clone() + iinterval.clone();
    }

    dbg!(combi_interval);
}
