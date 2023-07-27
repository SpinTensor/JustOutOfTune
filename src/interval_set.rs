use itertools::Itertools;

use crate::intervals::Interval;
use crate::interval_sequences::IntervalSequence;

#[derive(Debug, PartialEq, Clone)]
pub struct IntervalSet {
    n_major_third: i32,
    n_perfect_fourth: i32,
    n_perfect_fifth: i32
}

impl IntervalSet {
    pub fn new_empty() -> Self {
        IntervalSet::new_from_vals(0, 0, 0)
    }

    pub fn new_from_vals(n_major_third: i32, n_perfect_fourth: i32, n_perfect_fifth: i32) -> Self {
        Self {n_major_third, n_perfect_fourth, n_perfect_fifth}
    }

    pub fn new_with_hstep(half_steps: i32) -> Self {
        let mut interval_set = IntervalSet::new_empty();
        'limit: for maxval in 0.. {
            // construct an iterator that goes 0, 1, -1, 2, -2, ...
            let signed_iterator = (-maxval+1..1).rev().interleave(1..maxval);
            // the third counter is always positive in order to prohibit simple inversions of sets
            for ithird in signed_iterator.clone() {
                for ifourth in signed_iterator.clone() {
                    for ififth in signed_iterator.clone() {
                        let total_interval = Interval::major_third() * ithird
                                           + Interval::perfect_fourth() * ifourth
                                           + Interval::perfect_fifth() * ififth;
                        if total_interval.half_tone_steps == half_steps {
                            interval_set.n_major_third = ithird;
                            interval_set.n_perfect_fourth = ifourth;
                            interval_set.n_perfect_fifth = ififth;
                            break 'limit;
                        }
                    }
                }
            }
        }
        interval_set
    }

    pub fn new_with_frequency_scale() -> (Self, Self) {
        let mut interval_sets = (IntervalSet::new_empty(), IntervalSet::new_empty());
        // search for the downcaling set
        'limit: for maxval in 0.. {
            // construct an iterator that goes 0, 1, -1, 2, -2, ...
            let signed_iterator = (-maxval+1..1).rev().interleave(1..maxval);
            for ithird in signed_iterator.clone() {
                for ifourth in signed_iterator.clone() {
                    for ififth in signed_iterator.clone() {
                        // skip the empty set
                        if ithird == 0 && ifourth == 0 && ififth == 0 {
                            continue;
                        }
                        let total_interval = Interval::major_third() * ithird
                                           + Interval::perfect_fourth() * ifourth
                                           + Interval::perfect_fifth() * ififth;
                        if total_interval.half_tone_steps == 0 &&
                           total_interval.frequency_scale < 1 {
                            interval_sets.0.n_major_third = ithird;
                            interval_sets.0.n_perfect_fourth = ifourth;
                            interval_sets.0.n_perfect_fifth = ififth;
                            break 'limit;
                        }
                    }
                }
            }
        }
        // search for the upscaling set
        'limit: for maxval in 0.. {
            // construct an iterator that goes 0, 1, -1, 2, -2, ...
            let signed_iterator = (-maxval+1..1).rev().interleave(1..maxval);
            for ithird in signed_iterator.clone() {
                for ifourth in signed_iterator.clone() {
                    for ififth in signed_iterator.clone() {
                        // skip the empty set
                        if ithird == 0 && ifourth == 0 && ififth == 0 {
                            continue;
                        }
                        let total_interval = Interval::major_third() * ithird
                                           + Interval::perfect_fourth() * ifourth
                                           + Interval::perfect_fifth() * ififth;
                        if total_interval.half_tone_steps == 0 &&
                           total_interval.frequency_scale > 1 &&
                           ithird != -interval_sets.0.n_major_third &&
                           ifourth != -interval_sets.0.n_perfect_fourth &&
                           ififth != -interval_sets.0.n_perfect_fifth {
                            interval_sets.1.n_major_third = ithird;
                            interval_sets.1.n_perfect_fourth = ifourth;
                            interval_sets.1.n_perfect_fifth = ififth;
                            break 'limit;
                        }
                    }
                }
            }
        }
        interval_sets
    }
}

#[test]
fn search_interval_sets_with_hstep() {
    for hstep in -10..10 {
        assert_eq!(IntervalSet::new_with_hstep(hstep).to_interval_sequence().half_tone_steps(), hstep);
    }
}
#[test]
fn search_interval_sets_with_frequency_scale() {
    let interval_sets = IntervalSet::new_with_frequency_scale();
    assert!(interval_sets.0.to_interval_sequence().half_tone_steps() == 0);
    assert!(interval_sets.0.to_interval_sequence().frequency_scale() < 1);
    assert!(interval_sets.1.to_interval_sequence().half_tone_steps() == 0);
    assert!(interval_sets.1.to_interval_sequence().frequency_scale() > 1);
}

impl IntervalSet {
    pub fn to_interval_sequence(&self) -> IntervalSequence {
        let mut interval_seq = IntervalSequence::new();
        //major_thirds
        if self.n_major_third > 0 {
            for _ in 0..self.n_major_third {
                interval_seq = interval_seq + Interval::major_third();
            }
        } else {
            for _ in 0..self.n_major_third.abs() {
                interval_seq = interval_seq + (-Interval::major_third());
            }
        }
        //perfect_fourth
        if self.n_perfect_fourth > 0 {
            for _ in 0..self.n_perfect_fourth {
                interval_seq = interval_seq + Interval::perfect_fourth();
            }
        } else {
            for _ in 0..self.n_perfect_fourth.abs() {
                interval_seq = interval_seq + (-Interval::perfect_fourth());
            }
        }
        //perfect_fifth
        if self.n_perfect_fifth > 0 {
            for _ in 0..self.n_perfect_fifth {
                interval_seq = interval_seq + Interval::perfect_fifth();
            }   
        } else {
            for _ in 0..self.n_perfect_fifth.abs() {
                interval_seq = interval_seq + (-Interval::perfect_fifth());
            }   
        }

        interval_seq
    }
}

#[test]
fn to_interval_sequence() {
    let interval_set = IntervalSet::new_from_vals(2, 3, -2);
    let ref_sequence = IntervalSequence {
        intervals: vec![Interval::major_third(), Interval::major_third(),
                        Interval::perfect_fourth(), Interval::perfect_fourth(), Interval::perfect_fourth(),
                        -Interval::perfect_fifth(),-Interval::perfect_fifth()]
    };
    assert_eq!(interval_set.to_interval_sequence(), ref_sequence);
    let interval_set = IntervalSet::new_from_vals(-2, -3, 2);
    let ref_sequence = IntervalSequence {
        intervals: vec![-Interval::major_third(), -Interval::major_third(),
                        -Interval::perfect_fourth(), -Interval::perfect_fourth(), -Interval::perfect_fourth(),
                        Interval::perfect_fifth(),Interval::perfect_fifth()]
    };
    assert_eq!(interval_set.to_interval_sequence(), ref_sequence);
}


