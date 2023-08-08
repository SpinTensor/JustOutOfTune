use itertools::Itertools;
use num_traits::Pow;
use rug::Rational;
use crate::just_intervals::JustInterval;
use crate::interval_sequences::IntervalSequence;
use crate::vector_distributor::distribute;

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
                        let total_half_steps = JustInterval::MajorThird.get_half_steps() * ithird
                                             + JustInterval::PerfectFourth.get_half_steps() * ifourth
                                             + JustInterval::PerfectFifth.get_half_steps() * ififth;
                        if total_half_steps == half_steps {
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

    pub fn new_with_freq_scale() -> (Self, Self) {
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
                        let total_half_steps = JustInterval::MajorThird.get_half_steps() * ithird
                                             + JustInterval::PerfectFourth.get_half_steps() * ifourth
                                             + JustInterval::PerfectFifth.get_half_steps() * ififth;
                        let total_freq_scale = JustInterval::MajorThird.get_freq_scale().pow(ithird)
                                             * JustInterval::PerfectFourth.get_freq_scale().pow(ifourth)
                                             * JustInterval::PerfectFifth.get_freq_scale().pow(ififth);

                        if total_half_steps == 0 &&
                           total_freq_scale < 1 {
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
                        let total_half_steps = JustInterval::MajorThird.get_half_steps() * ithird
                                             + JustInterval::PerfectFourth.get_half_steps() * ifourth
                                             + JustInterval::PerfectFifth.get_half_steps() * ififth;
                        let total_freq_scale = JustInterval::MajorThird.get_freq_scale().pow(ithird)
                                             * JustInterval::PerfectFourth.get_freq_scale().pow(ifourth)
                                             * JustInterval::PerfectFifth.get_freq_scale().pow(ififth);
                        if total_half_steps == 0 &&
                           total_freq_scale > 1 &&
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

    pub fn get_half_steps(&self) -> i32 {
        JustInterval::MajorThird.get_half_steps() * self.n_major_third +
        JustInterval::PerfectFourth.get_half_steps() * self.n_perfect_fourth +
        JustInterval::PerfectFifth.get_half_steps() * self.n_perfect_fifth
    }

    pub fn get_freq_scale(&self) -> Rational {
        JustInterval::MajorThird.get_freq_scale().pow(self.n_major_third) *
        JustInterval::PerfectFourth.get_freq_scale().pow(self.n_perfect_fourth) *
        JustInterval::PerfectFifth.get_freq_scale().pow(self.n_perfect_fifth)
    }

    pub fn num_intervals(&self) -> usize {
        (self.n_major_third.abs()+self.n_perfect_fourth.abs()+self.n_perfect_fifth.abs()) as usize
    }

    pub fn add(&mut self, other: &Self) {
        self.n_major_third += other.n_major_third;
        self.n_perfect_fourth += other.n_perfect_fourth;
        self.n_perfect_fifth += other.n_perfect_fifth;
    }

    pub fn to_interval_sequence(&self) -> IntervalSequence {
        let mut interval_seq = IntervalSequence::new();
        //major_thirds
        if self.n_major_third > 0 {
            for _ in 0..self.n_major_third {
                interval_seq.add_interval(JustInterval::MajorThird);
            }
        } else {
            for _ in 0..self.n_major_third.abs() {
                interval_seq.add_interval(JustInterval::IMajorThird);
            }
        }
        //perfect_fourth
        if self.n_perfect_fourth > 0 {
            for _ in 0..self.n_perfect_fourth {
                interval_seq.add_interval(JustInterval::PerfectFourth);
            }
        } else {
            for _ in 0..self.n_perfect_fourth.abs() {
                interval_seq.add_interval(JustInterval::IPerfectFourth);
            }
        }
        //perfect_fifth
        if self.n_perfect_fifth > 0 {
            for _ in 0..self.n_perfect_fifth {
                interval_seq.add_interval(JustInterval::PerfectFifth);
            }   
        } else {
            for _ in 0..self.n_perfect_fifth.abs() {
                interval_seq.add_interval(JustInterval::IPerfectFifth);
            }   
        }
        distribute(&mut interval_seq.intervals);
        interval_seq
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_interval_sets_with_hstep() {
        for hstep in -10..10 {
            assert_eq!(IntervalSet::new_with_hstep(hstep).to_interval_sequence().get_half_steps(), hstep);
        }
    }
    #[test]
    fn search_interval_sets_with_freq_scale() {
        let interval_sets = IntervalSet::new_with_freq_scale();
        assert!(interval_sets.0.to_interval_sequence().get_half_steps() == 0);
        assert!(interval_sets.0.to_interval_sequence().get_freq_scale() < 1);
        assert!(interval_sets.1.to_interval_sequence().get_half_steps() == 0);
        assert!(interval_sets.1.to_interval_sequence().get_freq_scale() > 1);
    }

    #[test]
    fn add_interval_sets() {
        let mut set = IntervalSet::new_from_vals(1,2,3);
        set.add(&IntervalSet::new_from_vals(1,2,3));
        let ref_set = IntervalSet::new_from_vals(2,4,6);
        assert_eq!(set, ref_set);

        let mut set = IntervalSet::new_from_vals(1,2,3);
        set.add(&IntervalSet::new_from_vals(-1,-2,-3));
        let ref_set = IntervalSet::new_from_vals(0,0,0);
        assert_eq!(set, ref_set);

        let mut set = IntervalSet::new_from_vals(1,2,3);
        set.add(&IntervalSet::new_from_vals(-2,-4,-6));
        let ref_set = IntervalSet::new_from_vals(-1,-2,-3);
        assert_eq!(set, ref_set);
    }

    #[test]
    fn get_half_steps() {
        assert_eq!(IntervalSet::new_from_vals(1,2,2).get_half_steps(), 28);
    }

    #[test]
    fn get_freq_scale() {
        assert_eq!(IntervalSet::new_from_vals(1,2,-2).get_freq_scale(), Rational::from((80,81)));
    }
    
    #[test]
    fn to_interval_sequence() {
        let interval_set = IntervalSet::new_from_vals(2, 3, -2);
        let mut ref_sequence = IntervalSequence::new();
        ref_sequence.add_interval(JustInterval::PerfectFourth);
        ref_sequence.add_interval(JustInterval::MajorThird);
        ref_sequence.add_interval(JustInterval::IPerfectFifth);
        ref_sequence.add_interval(JustInterval::PerfectFourth);
        ref_sequence.add_interval(JustInterval::MajorThird);
        ref_sequence.add_interval(JustInterval::IPerfectFifth);
        ref_sequence.add_interval(JustInterval::PerfectFourth);
        assert_eq!(interval_set.to_interval_sequence(), ref_sequence);
    
        let interval_set = IntervalSet::new_from_vals(-2, -3, 2);
        let mut ref_sequence = IntervalSequence::new();
        ref_sequence.add_interval(JustInterval::IPerfectFourth);
        ref_sequence.add_interval(JustInterval::IMajorThird);
        ref_sequence.add_interval(JustInterval::PerfectFifth);
        ref_sequence.add_interval(JustInterval::IPerfectFourth);
        ref_sequence.add_interval(JustInterval::IMajorThird);
        ref_sequence.add_interval(JustInterval::PerfectFifth);
        ref_sequence.add_interval(JustInterval::IPerfectFourth);
        assert_eq!(interval_set.to_interval_sequence(), ref_sequence);
    }
}
