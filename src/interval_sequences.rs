use std::ops::{Add, AddAssign};
use rug::Rational;
use crate::intervals::Interval;
use crate::notes::Note;

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

    pub fn clean(&self) -> Self {
        let nthird = self.intervals.iter().filter(|&x| *x == Interval::major_third()).count();
        let nithird = self.intervals.iter().filter(|&x| *x == (-Interval::major_third())).count();
        let nfourth = self.intervals.iter().filter(|&x| *x == Interval::perfect_fourth()).count();
        let nifourth = self.intervals.iter().filter(|&x| *x == (-Interval::perfect_fourth())).count();
        let nfifth = self.intervals.iter().filter(|&x| *x == Interval::perfect_fifth()).count();
        let nififth = self.intervals.iter().filter(|&x| *x == (-Interval::perfect_fifth())).count();
        let mut cleaned_sequence = IntervalSequence::new();
        if nthird > nithird {
            for _ in nithird..nthird {
                cleaned_sequence = cleaned_sequence + Interval::major_third();
            }
        } else {
            for _ in nthird.. nithird {
                cleaned_sequence = cleaned_sequence + (-Interval::major_third());
            }
        }
        if nfourth > nifourth {
            for _ in nifourth..nfourth {
                cleaned_sequence = cleaned_sequence + Interval::perfect_fourth();
            }
        } else {
            for _ in nfourth.. nifourth {
                cleaned_sequence = cleaned_sequence + (-Interval::perfect_fourth());
            }
        }
        if nfifth > nififth {
            for _ in nififth..nfifth {
                cleaned_sequence = cleaned_sequence + Interval::perfect_fifth();
            }
        } else {
            for _ in nfifth.. nififth {
                cleaned_sequence = cleaned_sequence + (-Interval::perfect_fifth());
            }
        }

        cleaned_sequence
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

#[test]
fn clean() {
    let sequence = IntervalSequence::new()
                   + Interval::major_third()
                   + Interval::major_third()
                   + (-Interval::perfect_fourth())
                   + Interval::perfect_fifth()
                   + Interval::perfect_fifth()
                   + Interval::perfect_fifth()
                   + (-Interval::perfect_fifth())
                   + (-Interval::major_third())
                   + Interval::perfect_fourth();
    let ref_sequence = IntervalSequence::new()
                     + Interval::major_third()
                     + Interval::perfect_fifth()
                     + Interval::perfect_fifth();
    assert_eq!(sequence.clean(), ref_sequence);
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
    pub fn to_notes(&self, startingnote: Note) -> Vec<Note> {
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
    let startingnote = Note::new("C", 3);
    let sequence = IntervalSequence::new()
                 + Interval::major_third()
                 + (-Interval::perfect_fourth())
                 + Interval::perfect_fifth();

    let note_sequence = sequence.to_notes(startingnote);
    assert_eq!(note_sequence, vec![Note::new("C", 3), Note::new("E", 3), Note::new("B", 2), Note::new("F#", 3)]);
}
