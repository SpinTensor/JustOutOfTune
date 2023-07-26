use crate::intervals::Interval;
use crate::interval_sequences::IntervalSequence;

#[derive(Debug, PartialEq)]
pub struct IntervalSet {
    n_major_third: i32,
    n_perfect_fourth: i32,
    n_perfect_fifth: i32
}

impl IntervalSet {
    pub fn new() -> Self {
        Self {
            n_major_third: 0,
            n_perfect_fourth: 0,
            n_perfect_fifth: 0
        }
    }

    pub fn to_interval_sequence(&self) -> IntervalSequence {
        let mut interval_seq = IntervalSequence::new();
        //major_thirds
        if self.n_major_third > 0 {
            for iinterval in 0..self.n_major_third {
                interval_seq = interval_seq + Interval::major_third();
            }
        } else {
            for iinterval in 0..self.n_major_third.abs() {
                interval_seq = interval_seq + (-Interval::major_third());
            }
        }
        //perfect_fourth
        if self.n_perfect_fourth > 0 {
            for iinterval in 0..self.n_perfect_fourth {
                interval_seq = interval_seq + Interval::perfect_fourth();
            }
        } else {
            for iinterval in 0..self.n_perfect_fourth.abs() {
                interval_seq = interval_seq + (-Interval::perfect_fourth());
            }
        }
        //perfect_fifth
        if self.n_perfect_fifth > 0 {
            for iinterval in 0..self.n_perfect_fifth {
                interval_seq = interval_seq + Interval::perfect_fifth();
            }   
        } else {
            for iinterval in 0..self.n_perfect_fifth.abs() {
                interval_seq = interval_seq + (-Interval::perfect_fifth());
            }   
        }

        interval_seq
    }
}

#[test]
fn to_interval_sequence() {
    let interval_set = IntervalSet {
        n_major_third: 2,
        n_perfect_fourth: 3,
        n_perfect_fifth: -2,
    };
    let ref_sequence = IntervalSequence {
        intervals: vec![Interval::major_third(), Interval::major_third(),
                        Interval::perfect_fourth(), Interval::perfect_fourth(), Interval::perfect_fourth(),
                        -Interval::perfect_fifth(),-Interval::perfect_fifth()]
    };
    assert_eq!(interval_set.to_interval_sequence(), ref_sequence);
    let interval_set = IntervalSet {
        n_major_third: -2,
        n_perfect_fourth: -3,
        n_perfect_fifth: 2,
    };
    let ref_sequence = IntervalSequence {
        intervals: vec![-Interval::major_third(), -Interval::major_third(),
                        -Interval::perfect_fourth(), -Interval::perfect_fourth(), -Interval::perfect_fourth(),
                        Interval::perfect_fifth(),Interval::perfect_fifth()]
    };
    assert_eq!(interval_set.to_interval_sequence(), ref_sequence);
}
