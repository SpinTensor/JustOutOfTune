use std::ops::{Add, AddAssign};
use rug::Rational;
use crate::intervals::Interval;
use crate::notes::Notes;

#[derive(Debug, PartialEq, Clone)]
pub struct IntervalSequence {
    pub intervals: Vec<Interval>
}

impl IntervalSequence {
    pub fn new() -> Self {
        Self {
            intervals: Vec::new()
        }
    }

    pub fn len(&self) -> usize {
        self.intervals.len()
    }

    pub fn half_tone_steps(&self) -> i32 {
        let mut interval = Interval::new(0, (1,1));
        for iinterval in self.intervals.iter() {
            interval += iinterval.clone();
        }
        interval.half_tone_steps
    }

    pub fn frequency_scale(&self) -> Rational {
        let mut interval = Interval::new(0, (1,1));
        for iinterval in self.intervals.iter() {
            interval += iinterval.clone();
        }
        interval.frequency_scale
    }
}

#[test]
fn length() {
    let mut sequence = IntervalSequence::new();
    assert_eq!(sequence.len(), 0);
    sequence = sequence + Interval::major_third();
    assert_eq!(sequence.len(), 1);
    sequence = sequence + Interval::perfect_fourth();
    assert_eq!(sequence.len(), 2);
    sequence = sequence + Interval::perfect_fifth();
    assert_eq!(sequence.len(), 3);
}
#[test]
fn half_tone_steps() {
    let sequence = IntervalSequence::new()
                 + Interval::major_third()
                 + (-Interval::perfect_fourth().clone())
                 + Interval::perfect_fifth();
    assert_eq!(sequence.half_tone_steps(), 6);
}
#[test]
fn frequency_scale() {
    let sequence = IntervalSequence::new()
                 + Interval::major_third()
                 + (-Interval::perfect_fourth())
                 + Interval::perfect_fifth();
    assert_eq!(sequence.frequency_scale(), Rational::from((45,32)));
}

impl Add<Interval> for IntervalSequence {
    type Output = Self;

    fn add(self, other: Interval) -> Self::Output {
        let mut new_interval = self.clone();
        new_interval.intervals.push(other);
        new_interval
    }
}
#[test]
fn addition() {
    let mut sequence = IntervalSequence::new();
    sequence = sequence + Interval::major_third();
    sequence = sequence + Interval::perfect_fourth();
    sequence = sequence + Interval::perfect_fifth();
    let ref_sequence = IntervalSequence {
        intervals: vec![Interval::major_third(), Interval::perfect_fourth(), Interval::perfect_fifth()]
    };
    assert_eq!(sequence, ref_sequence);
}

impl AddAssign for IntervalSequence {
    fn add_assign(&mut self, other: Self) {
        for iinterval in other.intervals.iter() {
            self.intervals.push(iinterval.clone());
        }
    }
}
#[test]
fn assign_addition() {
    let mut sequence = IntervalSequence::new();
    sequence = sequence + Interval::major_third();
    sequence = sequence + Interval::perfect_fourth();
    sequence = sequence + Interval::perfect_fifth();
    let mut other_sequence = IntervalSequence::new();
    other_sequence = other_sequence + Interval::perfect_fourth();
    other_sequence = other_sequence + Interval::perfect_fifth();
    other_sequence = other_sequence + Interval::major_third();
    sequence += other_sequence.clone();
    let ref_sequence = IntervalSequence {
        intervals: vec![Interval::major_third(), Interval::perfect_fourth(), Interval::perfect_fifth(),
                        Interval::perfect_fourth(), Interval::perfect_fifth(), Interval::major_third()]
    };
    assert_eq!(sequence, ref_sequence);
}

impl IntervalSequence {
    pub fn to_notes(&self, startingnote: Notes) -> Vec<Notes> {
        let mut total_interval = Interval::new(0, (1,1));
        let mut note_sequence = Vec::new();
        note_sequence.push(startingnote);

        for iinterval in self.intervals.iter() {
            total_interval += iinterval.clone();
            let shifted_noted = startingnote.shift_by_interval(total_interval.clone());
            note_sequence.push(shifted_noted);
        }
        note_sequence
    }
}
#[test]
fn to_note_squence() {
    let startingnote = Notes::C;
    let sequence = IntervalSequence::new()
                 + Interval::major_third()
                 + (-Interval::perfect_fourth())
                 + Interval::perfect_fifth();

    let note_sequence = sequence.to_notes(startingnote);
    assert_eq!(note_sequence, vec![Notes::C, Notes::E, Notes::B, Notes::FSharp]);
}
