mod notes;
mod intervals;
mod interval_sequences;
mod interval_set;

use clap::Parser;
use notes::Notes;
use interval_sequences::IntervalSequence;
use interval_set::IntervalSet;

/// Compute just intonated interval sequences that drift in tuning
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Frequency scaling parameter
    #[arg(short, long, default_value_t = 1.0)]
    freq_scale: f64,

    /// Error of freq_scale in cents (1/100 half step)
    #[arg(short, long, default_value_t = 1.0)]
    err_freq_scale: f64,

    /// Number of half steps
    #[arg(short, long, default_value_t = 0)]
    nhalf_steps: i32,

}

fn main() {
    // command line parsing
    let args = Args::parse();

    println!("Starting out-of-tune sequence search with:");
    println!("   Number of half steps:      {:6}", args.nhalf_steps);
    println!("   Target frequency scaling:  {:6.1}", args.freq_scale);
    println!("   Max scaling error (cents): {:6.1}", args.err_freq_scale);
    println!();

    let mut interval_seq = IntervalSequence::new();

    // push the half step optimized interval set to the interval list
    println!("Searching for half step satisfying sequence:");
    let hstep_satis_seq = IntervalSet::new_with_hstep(args.nhalf_steps).to_interval_sequence();
    print!("   Number of half steps:    {:10} = ", hstep_satis_seq.half_tone_steps());
    for iinterval in &hstep_satis_seq.intervals {
        print!("{:4}", iinterval.half_tone_steps);
    }
    println!();
    print!("   frequency scaling:       {:10} = ", hstep_satis_seq.frequency_scale());
    for iinterval in &hstep_satis_seq.intervals {
        print!("{:4}", iinterval.frequency_scale);
    }
    println!();
    println!();
    interval_seq += hstep_satis_seq;

    // Create the two scaling interval sets
    println!("Searching for scaling sequences:");
    let scaling_seqs = IntervalSet::new_with_frequency_scale();
    let scaling_seqs = (scaling_seqs.0.to_interval_sequence(), scaling_seqs.1.to_interval_sequence());
    println!("   Downscaling sequence:");
    print!("      Number of half steps: {:10} = ", scaling_seqs.0.half_tone_steps());
    for iinterval in &scaling_seqs.0.intervals {
        print!("{:4}", iinterval.half_tone_steps);
    }
    println!();
    print!("      frequency scaling:    {:10} = ", scaling_seqs.0.frequency_scale());
    for iinterval in &scaling_seqs.0.intervals {
        print!("{:4}", iinterval.frequency_scale);
    }
    println!();
    println!("   Upscaling sequence:");
    print!("      Number of half steps: {:10} = ", scaling_seqs.1.half_tone_steps());
    for iinterval in &scaling_seqs.1.intervals {
        print!("{:4}", iinterval.half_tone_steps);
    }
    println!();
    print!("      frequency scaling:    {:10} = ", scaling_seqs.1.frequency_scale());
    for iinterval in &scaling_seqs.1.intervals {
        print!("{:4}", iinterval.frequency_scale);
    }
    println!();

    // compute the scaling error
    let to_cent = | x: f64 | -> f64 {1200.0*x.abs().log2()};
    let target_freq_scale_cent = to_cent(args.freq_scale);
    let mut sequence_freq_scale_cent = to_cent(interval_seq.frequency_scale().to_f64());
    let mut scale_err_cent = (target_freq_scale_cent-sequence_freq_scale_cent).abs();

    sequence_freq_scale_cent = to_cent(interval_seq.frequency_scale().to_f64());
    scale_err_cent = (target_freq_scale_cent-sequence_freq_scale_cent).abs();
    loop {
        let freq_scale = interval_seq.frequency_scale();
        if freq_scale == args.freq_scale ||
           scale_err_cent <= args.err_freq_scale {
            break;
        } else if freq_scale > args.freq_scale {
            // add interval sets that add 0 half steps until the freq_scale is smaller than the target
            interval_seq += scaling_seqs.0.clone();
        } else {
            // add interval sets that add 0 half steps until the freq_scale is larger than the target
            interval_seq += scaling_seqs.1.clone();
        }
        sequence_freq_scale_cent = to_cent(interval_seq.frequency_scale().to_f64());
        scale_err_cent = (target_freq_scale_cent-sequence_freq_scale_cent).abs();
    }

    dbg!(interval_seq.to_notes(Notes::C).len());
}
