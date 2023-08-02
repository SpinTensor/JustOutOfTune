use std::fmt;
use std::cmp::Ordering;
use crate::intervals::Interval;
use crate::notevalues::NoteValues;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Note {
    val: NoteValues,
    octave: i32
}

impl Note {
    pub fn new(s: &str, octave: i32) -> Self {
        let val = NoteValues::from_str(s).unwrap();
        Note {val, octave}
    }

    pub fn next(&self) -> Self {
        let new_octave = match self.val {
            NoteValues::B => self.octave+1,
            _ => self.octave
        };
        Self {
            val: self.val.next(),
            octave: new_octave
        }
    }

    pub fn prev(&self) -> Self {
        let new_octave = match self.val {
            NoteValues::C => self.octave-1,
            _ => self.octave
        };
        Self {
            val: self.val.prev(),
            octave: new_octave
        }
    }

    pub fn lower(&self, steps: u32) -> Self {
        match steps {
            0 => self.clone(),
            _ => self.prev().lower(steps-1)
        }
    }

    pub fn raise(&self, steps: u32) -> Self {
        match steps {
            0 => self.clone(),
            _ => self.next().raise(steps-1)
        }
    }

    pub fn shift(&self, steps: i32) -> Self {
        match steps.cmp(&0) {
            Ordering::Less => self.lower(-steps as u32),
            Ordering::Equal => self.clone(),
            Ordering::Greater => self.raise(steps as u32)
        }
    }

    pub fn shift_by_interval(&self, intv: Interval) -> Self {
        self.shift(intv.half_tone_steps)
    }

}

#[test]
fn next() {
    assert_eq!(Note::new("B", 2).next(), Note::new("C", 3));
    assert_eq!(Note::new("C", 3).next(), Note::new("C#", 3));
    assert_eq!(Note::new("C#",3).next(), Note::new("D", 3));
    assert_eq!(Note::new("D", 3).next(), Note::new("D#", 3));
    assert_eq!(Note::new("D#",3).next(), Note::new("E", 3));
    assert_eq!(Note::new("E", 3).next(), Note::new("F", 3));
    assert_eq!(Note::new("F", 3).next(), Note::new("F#", 3));
    assert_eq!(Note::new("F#",3).next(), Note::new("G", 3));
    assert_eq!(Note::new("G", 3).next(), Note::new("G#", 3));
    assert_eq!(Note::new("G#",3).next(), Note::new("A", 3));
    assert_eq!(Note::new("A", 3).next(), Note::new("A#", 3));
    assert_eq!(Note::new("A#",3).next(), Note::new("B", 3));
    assert_eq!(Note::new("B", 3).next(), Note::new("C", 4));
}

#[test]
fn prev() {
    assert_eq!(Note::new("C", 3).prev(), Note::new("B", 2));
    assert_eq!(Note::new("C#",3).prev(), Note::new("C", 3));
    assert_eq!(Note::new("D", 3).prev(), Note::new("C#",3));
    assert_eq!(Note::new("D#",3).prev(), Note::new("D", 3));
    assert_eq!(Note::new("E", 3).prev(), Note::new("D#",3));
    assert_eq!(Note::new("F", 3).prev(), Note::new("E", 3));
    assert_eq!(Note::new("F#",3).prev(), Note::new("F", 3));
    assert_eq!(Note::new("G", 3).prev(), Note::new("F#",3));
    assert_eq!(Note::new("G#",3).prev(), Note::new("G", 3));
    assert_eq!(Note::new("A", 3).prev(), Note::new("G#",3));
    assert_eq!(Note::new("A#",3).prev(), Note::new("A", 3));
    assert_eq!(Note::new("B", 3).prev(), Note::new("A#",3));
    assert_eq!(Note::new("C", 4).prev(), Note::new("B", 3));
}

#[test]
fn raise() {
    assert_eq!(Note::new("C", 3).raise(0),  Note::new("C", 3));
    assert_eq!(Note::new("C", 3).raise(1),  Note::new("C#", 3));
    assert_eq!(Note::new("C", 3).raise(2),  Note::new("D", 3));
    assert_eq!(Note::new("C", 3).raise(3),  Note::new("D#", 3));
    assert_eq!(Note::new("C", 3).raise(4),  Note::new("E", 3));
    assert_eq!(Note::new("C", 3).raise(5),  Note::new("F", 3));
    assert_eq!(Note::new("C", 3).raise(6),  Note::new("F#", 3));
    assert_eq!(Note::new("C", 3).raise(7),  Note::new("G", 3));
    assert_eq!(Note::new("C", 3).raise(8),  Note::new("G#", 3));
    assert_eq!(Note::new("C", 3).raise(9),  Note::new("A", 3));
    assert_eq!(Note::new("C", 3).raise(10), Note::new("A#", 3));
    assert_eq!(Note::new("C", 3).raise(11), Note::new("B", 3));
    assert_eq!(Note::new("C", 3).raise(12), Note::new("C", 4));
    assert_eq!(Note::new("C", 3).raise(13), Note::new("C#", 4));
}

#[test]
fn lower() {
    assert_eq!(Note::new("C", 3).lower(0),  Note::new("C", 3));
    assert_eq!(Note::new("C", 3).lower(1),  Note::new("B", 2));
    assert_eq!(Note::new("C", 3).lower(2),  Note::new("A#", 2));
    assert_eq!(Note::new("C", 3).lower(3),  Note::new("A", 2));
    assert_eq!(Note::new("C", 3).lower(4),  Note::new("G#", 2));
    assert_eq!(Note::new("C", 3).lower(5),  Note::new("G", 2));
    assert_eq!(Note::new("C", 3).lower(6),  Note::new("F#", 2));
    assert_eq!(Note::new("C", 3).lower(7),  Note::new("F", 2));
    assert_eq!(Note::new("C", 3).lower(8),  Note::new("E", 2));
    assert_eq!(Note::new("C", 3).lower(9),  Note::new("D#", 2));
    assert_eq!(Note::new("C", 3).lower(10), Note::new("D", 2));
    assert_eq!(Note::new("C", 3).lower(11), Note::new("C#", 2));
    assert_eq!(Note::new("C", 3).lower(12), Note::new("C", 2));
    assert_eq!(Note::new("C", 3).lower(13), Note::new("B", 1));
}

#[test]
fn shift() {
    assert_eq!(Note::new("C", 3).shift(0), Note::new("C", 3));
    assert_eq!(Note::new("C", 3).shift(5), Note::new("F", 3));
    assert_eq!(Note::new("C", 3).shift(-5), Note::new("G", 2));
}

#[test]
fn shift_by_interval() {
    assert_eq!(Note::new("C", 3).shift_by_interval(Interval::perfect_fifth()), Note::new("G", 3));
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.val.to_str().len() == 1 {
            write!(f, " {}{}", self.val.to_str(), self.octave)
        } else {
            write!(f, "{}{}", self.val.to_str(), self.octave)
        }

    }
}
