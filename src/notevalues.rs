use std::cmp::Ordering;
use crate::intervals::Interval;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NoteValues {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B
}

impl NoteValues {
    pub fn from_str(s: &str) -> Result<Self, &'static str> {
        match s.to_lowercase().as_str() {
            "cb" => Ok(NoteValues::B),
            "c"  => Ok(NoteValues::C),
            "c#" => Ok(NoteValues::CSharp),
            "db" => Ok(NoteValues::CSharp),
            "d"  => Ok(NoteValues::D),
            "d#" => Ok(NoteValues::DSharp),
            "eb" => Ok(NoteValues::DSharp),
            "e"  => Ok(NoteValues::E),
            "e#" => Ok(NoteValues::F),
            "fb" => Ok(NoteValues::E),
            "f"  => Ok(NoteValues::F),
            "f#" => Ok(NoteValues::FSharp),
            "gb" => Ok(NoteValues::FSharp),
            "g"  => Ok(NoteValues::G),
            "g#" => Ok(NoteValues::GSharp),
            "ab" => Ok(NoteValues::GSharp),
            "a"  => Ok(NoteValues::A),
            "a#" => Ok(NoteValues::ASharp),
            "bb" => Ok(NoteValues::ASharp),
            "b"  => Ok(NoteValues::B),
            "b#" => Ok(NoteValues::C),
            _ => Err("Invalid Note Name")
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            NoteValues::C => "C",
            NoteValues::CSharp => "C#",
            NoteValues::D => "D",
            NoteValues::DSharp => "D#",
            NoteValues::E => "E",
            NoteValues::F => "F",
            NoteValues::FSharp => "F#",
            NoteValues::G => "G",
            NoteValues::GSharp => "G#",
            NoteValues::A => "A",
            NoteValues::ASharp => "A#",
            NoteValues::B => "B",
        }
    }
}
#[test]
fn from_str() {
    assert_eq!(NoteValues::from_str(&String::from("Cb")).unwrap(), NoteValues::B);
    assert_eq!(NoteValues::from_str(&String::from("C" )).unwrap(), NoteValues::C);
    assert_eq!(NoteValues::from_str(&String::from("C#")).unwrap(), NoteValues::CSharp);
    assert_eq!(NoteValues::from_str(&String::from("Db")).unwrap(), NoteValues::CSharp);
    assert_eq!(NoteValues::from_str(&String::from("D" )).unwrap(), NoteValues::D);
    assert_eq!(NoteValues::from_str(&String::from("D#")).unwrap(), NoteValues::DSharp);
    assert_eq!(NoteValues::from_str(&String::from("Eb")).unwrap(), NoteValues::DSharp);
    assert_eq!(NoteValues::from_str(&String::from("E" )).unwrap(), NoteValues::E);
    assert_eq!(NoteValues::from_str(&String::from("E#")).unwrap(), NoteValues::F);
    assert_eq!(NoteValues::from_str(&String::from("Fb")).unwrap(), NoteValues::E);
    assert_eq!(NoteValues::from_str(&String::from("F" )).unwrap(), NoteValues::F);
    assert_eq!(NoteValues::from_str(&String::from("F#")).unwrap(), NoteValues::FSharp);
    assert_eq!(NoteValues::from_str(&String::from("Gb")).unwrap(), NoteValues::FSharp);
    assert_eq!(NoteValues::from_str(&String::from("G" )).unwrap(), NoteValues::G);
    assert_eq!(NoteValues::from_str(&String::from("G#")).unwrap(), NoteValues::GSharp);
    assert_eq!(NoteValues::from_str(&String::from("Ab")).unwrap(), NoteValues::GSharp);
    assert_eq!(NoteValues::from_str(&String::from("A" )).unwrap(), NoteValues::A);
    assert_eq!(NoteValues::from_str(&String::from("A#")).unwrap(), NoteValues::ASharp);
    assert_eq!(NoteValues::from_str(&String::from("Bb")).unwrap(), NoteValues::ASharp);
    assert_eq!(NoteValues::from_str(&String::from("B" )).unwrap(), NoteValues::B);
    assert_eq!(NoteValues::from_str(&String::from("B#")).unwrap(), NoteValues::C);

    assert_eq!(NoteValues::from_str(&String::from("cb")).unwrap(), NoteValues::B);
    assert_eq!(NoteValues::from_str(&String::from("c" )).unwrap(), NoteValues::C);
    assert_eq!(NoteValues::from_str(&String::from("c#")).unwrap(), NoteValues::CSharp);
    assert_eq!(NoteValues::from_str(&String::from("db")).unwrap(), NoteValues::CSharp);
    assert_eq!(NoteValues::from_str(&String::from("d" )).unwrap(), NoteValues::D);
    assert_eq!(NoteValues::from_str(&String::from("d#")).unwrap(), NoteValues::DSharp);
    assert_eq!(NoteValues::from_str(&String::from("eb")).unwrap(), NoteValues::DSharp);
    assert_eq!(NoteValues::from_str(&String::from("e" )).unwrap(), NoteValues::E);
    assert_eq!(NoteValues::from_str(&String::from("e#")).unwrap(), NoteValues::F);
    assert_eq!(NoteValues::from_str(&String::from("fb")).unwrap(), NoteValues::E);
    assert_eq!(NoteValues::from_str(&String::from("f" )).unwrap(), NoteValues::F);
    assert_eq!(NoteValues::from_str(&String::from("f#")).unwrap(), NoteValues::FSharp);
    assert_eq!(NoteValues::from_str(&String::from("gb")).unwrap(), NoteValues::FSharp);
    assert_eq!(NoteValues::from_str(&String::from("g" )).unwrap(), NoteValues::G);
    assert_eq!(NoteValues::from_str(&String::from("g#")).unwrap(), NoteValues::GSharp);
    assert_eq!(NoteValues::from_str(&String::from("ab")).unwrap(), NoteValues::GSharp);
    assert_eq!(NoteValues::from_str(&String::from("a" )).unwrap(), NoteValues::A);
    assert_eq!(NoteValues::from_str(&String::from("a#")).unwrap(), NoteValues::ASharp);
    assert_eq!(NoteValues::from_str(&String::from("bb")).unwrap(), NoteValues::ASharp);
    assert_eq!(NoteValues::from_str(&String::from("b" )).unwrap(), NoteValues::B);
    assert_eq!(NoteValues::from_str(&String::from("b#")).unwrap(), NoteValues::C);
}
#[test]
fn to_str() {
    assert_eq!(NoteValues::C.to_str(), String::from("C"));
    assert_eq!(NoteValues::CSharp.to_str(), String::from("C#"));
    assert_eq!(NoteValues::D.to_str(), String::from("D"));
    assert_eq!(NoteValues::DSharp.to_str(), String::from("D#"));
    assert_eq!(NoteValues::E.to_str(), String::from("E"));
    assert_eq!(NoteValues::F.to_str(), String::from("F"));
    assert_eq!(NoteValues::FSharp.to_str(), String::from("F#"));
    assert_eq!(NoteValues::G.to_str(), String::from("G"));
    assert_eq!(NoteValues::GSharp.to_str(), String::from("G#"));
    assert_eq!(NoteValues::A.to_str(), String::from("A"));
    assert_eq!(NoteValues::ASharp.to_str(), String::from("A#"));
    assert_eq!(NoteValues::B.to_str(), String::from("B"));
}

impl NoteValues {
    pub fn next(&self) -> Self {
        match self {
            NoteValues::C =>      NoteValues::CSharp,
            NoteValues::CSharp => NoteValues::D,
            NoteValues::D =>      NoteValues::DSharp,
            NoteValues::DSharp => NoteValues::E,
            NoteValues::E =>      NoteValues::F,
            NoteValues::F =>      NoteValues::FSharp,
            NoteValues::FSharp => NoteValues::G,
            NoteValues::G =>      NoteValues::GSharp,
            NoteValues::GSharp => NoteValues::A,
            NoteValues::A =>      NoteValues::ASharp,
            NoteValues::ASharp => NoteValues::B,
            NoteValues::B =>      NoteValues::C
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            NoteValues::C =>      NoteValues::B,
            NoteValues::CSharp => NoteValues::C,
            NoteValues::D =>      NoteValues::CSharp,
            NoteValues::DSharp => NoteValues::D,   
            NoteValues::E =>      NoteValues::DSharp,
            NoteValues::F =>      NoteValues::E,   
            NoteValues::FSharp => NoteValues::F,      
            NoteValues::G =>      NoteValues::FSharp,
            NoteValues::GSharp => NoteValues::G,   
            NoteValues::A =>      NoteValues::GSharp,
            NoteValues::ASharp => NoteValues::A,   
            NoteValues::B =>      NoteValues::ASharp
        }
    }

    fn same(&self) -> Self {
        match self {
            NoteValues::C =>      NoteValues::C,
            NoteValues::CSharp => NoteValues::CSharp,
            NoteValues::D =>      NoteValues::D,
            NoteValues::DSharp => NoteValues::DSharp,
            NoteValues::E =>      NoteValues::E,
            NoteValues::F =>      NoteValues::F,
            NoteValues::FSharp => NoteValues::FSharp,
            NoteValues::G =>      NoteValues::G,
            NoteValues::GSharp => NoteValues::GSharp,
            NoteValues::A =>      NoteValues::A,
            NoteValues::ASharp => NoteValues::ASharp,
            NoteValues::B =>      NoteValues::B
        }
    }

    pub fn raise(&self, steps: u32) -> Self {
        match steps {
            0 => self.same(),
            _ => self.next().raise(steps-1)
        }
    }

    pub fn lower(&self, steps: u32) -> Self {
        match steps {
            0 => self.same(),
            _ => self.prev().lower(steps-1)
        }
    }

    pub fn shift(&self, steps: i32) -> Self {
        match steps.cmp(&0) {
            Ordering::Less => self.lower(-steps as u32),
            Ordering::Equal => self.same(),
            Ordering::Greater => self.raise(steps as u32)
        }
    }

    pub fn shift_by_interval(&self, intv: Interval) -> Self {
        self.shift(intv.half_tone_steps)
    }
}

#[test] 
fn raise() {
    assert_eq!(NoteValues::C.raise(0), NoteValues::C);
    assert_eq!(NoteValues::C.raise(1), NoteValues::CSharp);
    assert_eq!(NoteValues::C.raise(2), NoteValues::D);
    assert_eq!(NoteValues::C.raise(3), NoteValues::DSharp);
    assert_eq!(NoteValues::C.raise(4), NoteValues::E);
    assert_eq!(NoteValues::C.raise(5), NoteValues::F);
    assert_eq!(NoteValues::C.raise(6), NoteValues::FSharp);
    assert_eq!(NoteValues::C.raise(7), NoteValues::G);
    assert_eq!(NoteValues::C.raise(8), NoteValues::GSharp);
    assert_eq!(NoteValues::C.raise(9), NoteValues::A);
    assert_eq!(NoteValues::C.raise(10), NoteValues::ASharp);
    assert_eq!(NoteValues::C.raise(11), NoteValues::B);
    assert_eq!(NoteValues::C.raise(12), NoteValues::C);
    assert_eq!(NoteValues::C.raise(13), NoteValues::CSharp);
}

#[test]
fn lower() {
    assert_eq!(NoteValues::C.lower(0), NoteValues::C);
    assert_eq!(NoteValues::C.lower(1), NoteValues::B);
    assert_eq!(NoteValues::C.lower(2), NoteValues::ASharp);
    assert_eq!(NoteValues::C.lower(3), NoteValues::A);
    assert_eq!(NoteValues::C.lower(4), NoteValues::GSharp);
    assert_eq!(NoteValues::C.lower(5), NoteValues::G);
    assert_eq!(NoteValues::C.lower(6), NoteValues::FSharp);
    assert_eq!(NoteValues::C.lower(7), NoteValues::F);
    assert_eq!(NoteValues::C.lower(8), NoteValues::E);
    assert_eq!(NoteValues::C.lower(9), NoteValues::DSharp);
    assert_eq!(NoteValues::C.lower(10), NoteValues::D);
    assert_eq!(NoteValues::C.lower(11), NoteValues::CSharp);
    assert_eq!(NoteValues::C.lower(12), NoteValues::C);
    assert_eq!(NoteValues::C.lower(13), NoteValues::B);
}

#[test]
fn shift() {
    assert_eq!(NoteValues::C.shift(0), NoteValues::C);
    assert_eq!(NoteValues::C.shift(5), NoteValues::F);
    assert_eq!(NoteValues::C.shift(-5), NoteValues::G);
}

#[test]
fn shift_by_interval() {
    assert_eq!(NoteValues::C.shift_by_interval(Interval::perfect_fifth()), NoteValues::G);
}
