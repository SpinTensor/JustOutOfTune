mod just_intervals;
mod interval_sequences;
mod notevalues;
mod notes;
mod interval_set;
mod vector_distributor;

use clap::Parser;
use crate::notes::Note;
use crate::interval_set::IntervalSet;

/// Compute just intonated interval sequences that drift in tuning
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Frequency scaling parameter
    #[arg(long, default_value_t = 1.0)]
    freq_scale: f64,

    /// Error of freq_scale in cents (1/100 half step)
    #[arg(long, default_value_t = 1.0)]
    freq_scale_err: f64,

    /// Number of half steps
    #[arg(long, default_value_t = 0)]
    nhalf_steps: i32,

    /// Starting note name
    #[arg(long, default_value_t = String::from("C"))]
    starting_note: String,

    /// Starting octave
    #[arg(long, default_value_t = 3)]
    starting_octave: i32,

    /// Split notes for instruments
    #[arg(long, default_value_t = false)]
    split_note_sequence: bool,
}

fn main() {
    // command line parsing
    let args = Args::parse();

    println!("Starting out-of-tune sequence search with:");
    println!("   Number of half steps:      {:10}", args.nhalf_steps);
    println!("   Target frequency scaling:  {:10.3}", args.freq_scale);
    println!("   Max scaling error (cents): {:10.3}", args.freq_scale_err);
    println!("   Starting note and octave   {:>9}{:1}", args.starting_note, args.starting_octave);
    println!("   Split note sequence        {:>10}", args.split_note_sequence);
    println!();

    // push the half step optimized interval set to the interval list
    println!("Searching for half step satisfying sequence:");
    let hstep_satis_set = IntervalSet::new_with_hstep(args.nhalf_steps);
    let hstep_satis_seq = hstep_satis_set.to_interval_sequence();
    print!("   Number of half steps:    {:10} = ", hstep_satis_set.get_half_steps());
    for iinterval in &hstep_satis_seq.intervals {
        print!("{:4}", iinterval.get_half_steps());
    }
    println!();
    print!("   frequency scaling:       {:10} = ", hstep_satis_set.get_freq_scale());
    for iinterval in &hstep_satis_seq.intervals {
        print!("{:4}", iinterval.get_freq_scale());
    }
    println!();
    println!();

    // Create the two scaling interval sets
    println!("Searching for scaling sequences:");
    let scaling_sets = IntervalSet::new_with_freq_scale();
    let scaling_seqs = (scaling_sets.0.to_interval_sequence(), scaling_sets.1.to_interval_sequence());
    println!("   Downscaling sequence:");
    print!("      Number of half steps: {:10} = ", scaling_sets.0.get_half_steps());
    for iinterval in &scaling_seqs.0.intervals {
        print!("{:4}", iinterval.get_half_steps());
    }
    println!();
    print!("      frequency scaling:    {:10} = ", scaling_sets.0.get_freq_scale());
    for iinterval in &scaling_seqs.0.intervals {
        print!("{:4}", iinterval.get_freq_scale());
    }
    println!();
    println!("   Upscaling sequence:");
    print!("      Number of half steps: {:10} = ", scaling_sets.1.get_half_steps());
    for iinterval in &scaling_seqs.1.intervals {
        print!("{:4}", iinterval.get_half_steps());
    }
    println!();
    print!("      frequency scaling:    {:10} = ", scaling_sets.1.get_freq_scale());
    for iinterval in &scaling_seqs.1.intervals {
        print!("{:4}", iinterval.get_freq_scale());
    }
    println!();
    println!();

    // start building the interval set
    let mut interval_set = IntervalSet::new_empty();
    interval_set.add(&hstep_satis_set);
    // compute the scaling error
    let to_cent = | x: f64 | -> f64 {1200.0*x.abs().log2()};
    let target_freq_scale_cent = to_cent(args.freq_scale);
    let mut set_freq_scale_cent = to_cent(interval_set.get_freq_scale().to_f64());
    let mut scale_err_cent = (target_freq_scale_cent-set_freq_scale_cent).abs();

    loop {
        let freq_scale = interval_set.get_freq_scale();
        if freq_scale == args.freq_scale ||
           scale_err_cent <= args.freq_scale_err {
            break;
        } else if freq_scale > args.freq_scale {
            // add interval sets that add 0 half steps until the freq_scale is smaller than the target
            interval_set.add(&scaling_sets.0);
        } else {
            // add interval sets that add 0 half steps until the freq_scale is larger than the target
            interval_set.add(&scaling_sets.1);
        }
        set_freq_scale_cent = to_cent(interval_set.get_freq_scale().to_f64());
        scale_err_cent = (target_freq_scale_cent-set_freq_scale_cent).abs();
    }

    println!("Found Sequence:");
    println!("   Number of intervals:   {}", interval_set.num_intervals());
    println!("   Scaling frequency:     {:}", interval_set.get_freq_scale().to_f64());
    println!("   Scaling error (cents): {:}", scale_err_cent);

    let interval_seq = interval_set.to_interval_sequence();

    print!("List of notes that correspond to the interval sequence:");
    let note_sequence = interval_seq.to_notes(Note::new(&args.starting_note, args.starting_octave));
    for (inote, note) in note_sequence.iter().enumerate() {
        if inote % 20 == 0 {
            println!();
        }
        print!(" {}", note);
    }
    println!();

    if args.split_note_sequence {
        println!();
        print!("Instrument 1:");
        for (inote, note) in note_sequence.iter().enumerate() {
            if inote % 20 == 0 {
                println!();
            }
            if inote % 2 == 0 {
                print!(" {}", note);
            }
        }
        println!();

        println!();
        print!("Instrument 2:");
        for (inote, note) in note_sequence.iter().enumerate() {
            if inote % 20 == 0 {
                println!();
            }
            if (inote+1) % 2 == 0 {
                print!(" {}", note);
            }
        }
        println!();
    }
}
