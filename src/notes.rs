use std::cmp::Ordering;
use crate::intervals::Interval;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Notes {
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

impl Notes {
    pub fn from_str(s: &str) -> Result<Self, &'static str> {
        match s.to_lowercase().as_str() {
            "cb" => Ok(Notes::B),
            "c"  => Ok(Notes::C),
            "c#" => Ok(Notes::CSharp),
            "db" => Ok(Notes::CSharp),
            "d"  => Ok(Notes::D),
            "d#" => Ok(Notes::DSharp),
            "eb" => Ok(Notes::DSharp),
            "e"  => Ok(Notes::E),
            "e#" => Ok(Notes::F),
            "fb" => Ok(Notes::E),
            "f"  => Ok(Notes::F),
            "f#" => Ok(Notes::FSharp),
            "gb" => Ok(Notes::FSharp),
            "g"  => Ok(Notes::G),
            "g#" => Ok(Notes::GSharp),
            "ab" => Ok(Notes::GSharp),
            "a"  => Ok(Notes::A),
            "a#" => Ok(Notes::ASharp),
            "bb" => Ok(Notes::ASharp),
            "b"  => Ok(Notes::B),
            "b#" => Ok(Notes::C),
            _ => Err("Invalid Note Name")
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Notes::C => "C",
            Notes::CSharp => "C#",
            Notes::D => "D",
            Notes::DSharp => "D#",
            Notes::E => "E",
            Notes::F => "F",
            Notes::FSharp => "F#",
            Notes::G => "G",
            Notes::GSharp => "G#",
            Notes::A => "A",
            Notes::ASharp => "A#",
            Notes::B => "B",
        }
    }
}
#[test]
fn from_str() {
    assert_eq!(Notes::from_str(&String::from("Cb")).unwrap(), Notes::B);
    assert_eq!(Notes::from_str(&String::from("C" )).unwrap(), Notes::C);
    assert_eq!(Notes::from_str(&String::from("C#")).unwrap(), Notes::CSharp);
    assert_eq!(Notes::from_str(&String::from("Db")).unwrap(), Notes::CSharp);
    assert_eq!(Notes::from_str(&String::from("D" )).unwrap(), Notes::D);
    assert_eq!(Notes::from_str(&String::from("D#")).unwrap(), Notes::DSharp);
    assert_eq!(Notes::from_str(&String::from("Eb")).unwrap(), Notes::DSharp);
    assert_eq!(Notes::from_str(&String::from("E" )).unwrap(), Notes::E);
    assert_eq!(Notes::from_str(&String::from("E#")).unwrap(), Notes::F);
    assert_eq!(Notes::from_str(&String::from("Fb")).unwrap(), Notes::E);
    assert_eq!(Notes::from_str(&String::from("F" )).unwrap(), Notes::F);
    assert_eq!(Notes::from_str(&String::from("F#")).unwrap(), Notes::FSharp);
    assert_eq!(Notes::from_str(&String::from("Gb")).unwrap(), Notes::FSharp);
    assert_eq!(Notes::from_str(&String::from("G" )).unwrap(), Notes::G);
    assert_eq!(Notes::from_str(&String::from("G#")).unwrap(), Notes::GSharp);
    assert_eq!(Notes::from_str(&String::from("Ab")).unwrap(), Notes::GSharp);
    assert_eq!(Notes::from_str(&String::from("A" )).unwrap(), Notes::A);
    assert_eq!(Notes::from_str(&String::from("A#")).unwrap(), Notes::ASharp);
    assert_eq!(Notes::from_str(&String::from("Bb")).unwrap(), Notes::ASharp);
    assert_eq!(Notes::from_str(&String::from("B" )).unwrap(), Notes::B);
    assert_eq!(Notes::from_str(&String::from("B#")).unwrap(), Notes::C);

    assert_eq!(Notes::from_str(&String::from("cb")).unwrap(), Notes::B);
    assert_eq!(Notes::from_str(&String::from("c" )).unwrap(), Notes::C);
    assert_eq!(Notes::from_str(&String::from("c#")).unwrap(), Notes::CSharp);
    assert_eq!(Notes::from_str(&String::from("db")).unwrap(), Notes::CSharp);
    assert_eq!(Notes::from_str(&String::from("d" )).unwrap(), Notes::D);
    assert_eq!(Notes::from_str(&String::from("d#")).unwrap(), Notes::DSharp);
    assert_eq!(Notes::from_str(&String::from("eb")).unwrap(), Notes::DSharp);
    assert_eq!(Notes::from_str(&String::from("e" )).unwrap(), Notes::E);
    assert_eq!(Notes::from_str(&String::from("e#")).unwrap(), Notes::F);
    assert_eq!(Notes::from_str(&String::from("fb")).unwrap(), Notes::E);
    assert_eq!(Notes::from_str(&String::from("f" )).unwrap(), Notes::F);
    assert_eq!(Notes::from_str(&String::from("f#")).unwrap(), Notes::FSharp);
    assert_eq!(Notes::from_str(&String::from("gb")).unwrap(), Notes::FSharp);
    assert_eq!(Notes::from_str(&String::from("g" )).unwrap(), Notes::G);
    assert_eq!(Notes::from_str(&String::from("g#")).unwrap(), Notes::GSharp);
    assert_eq!(Notes::from_str(&String::from("ab")).unwrap(), Notes::GSharp);
    assert_eq!(Notes::from_str(&String::from("a" )).unwrap(), Notes::A);
    assert_eq!(Notes::from_str(&String::from("a#")).unwrap(), Notes::ASharp);
    assert_eq!(Notes::from_str(&String::from("bb")).unwrap(), Notes::ASharp);
    assert_eq!(Notes::from_str(&String::from("b" )).unwrap(), Notes::B);
    assert_eq!(Notes::from_str(&String::from("b#")).unwrap(), Notes::C);
}
#[test]
fn to_str() {
    assert_eq!(Notes::C.to_str(), String::from("C"));
    assert_eq!(Notes::CSharp.to_str(), String::from("C#"));
    assert_eq!(Notes::D.to_str(), String::from("D"));
    assert_eq!(Notes::DSharp.to_str(), String::from("D#"));
    assert_eq!(Notes::E.to_str(), String::from("E"));
    assert_eq!(Notes::F.to_str(), String::from("F"));
    assert_eq!(Notes::FSharp.to_str(), String::from("F#"));
    assert_eq!(Notes::G.to_str(), String::from("G"));
    assert_eq!(Notes::GSharp.to_str(), String::from("G#"));
    assert_eq!(Notes::A.to_str(), String::from("A"));
    assert_eq!(Notes::ASharp.to_str(), String::from("A#"));
    assert_eq!(Notes::B.to_str(), String::from("B"));
}

impl Notes {
    pub fn next(&self) -> Self {
        match self {
            Notes::C =>      Notes::CSharp,
            Notes::CSharp => Notes::D,
            Notes::D =>      Notes::DSharp,
            Notes::DSharp => Notes::E,
            Notes::E =>      Notes::F,
            Notes::F =>      Notes::FSharp,
            Notes::FSharp => Notes::G,
            Notes::G =>      Notes::GSharp,
            Notes::GSharp => Notes::A,
            Notes::A =>      Notes::ASharp,
            Notes::ASharp => Notes::B,
            Notes::B =>      Notes::C
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Notes::C =>      Notes::B,
            Notes::CSharp => Notes::C,
            Notes::D =>      Notes::CSharp,
            Notes::DSharp => Notes::D,   
            Notes::E =>      Notes::DSharp,
            Notes::F =>      Notes::E,   
            Notes::FSharp => Notes::F,      
            Notes::G =>      Notes::FSharp,
            Notes::GSharp => Notes::G,   
            Notes::A =>      Notes::GSharp,
            Notes::ASharp => Notes::A,   
            Notes::B =>      Notes::ASharp
        }
    }

    fn same(&self) -> Self {
        match self {
            Notes::C =>      Notes::C,
            Notes::CSharp => Notes::CSharp,
            Notes::D =>      Notes::D,
            Notes::DSharp => Notes::DSharp,
            Notes::E =>      Notes::E,
            Notes::F =>      Notes::F,
            Notes::FSharp => Notes::FSharp,
            Notes::G =>      Notes::G,
            Notes::GSharp => Notes::GSharp,
            Notes::A =>      Notes::A,
            Notes::ASharp => Notes::ASharp,
            Notes::B =>      Notes::B
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
    assert_eq!(Notes::C.raise(0), Notes::C);
    assert_eq!(Notes::C.raise(1), Notes::CSharp);
    assert_eq!(Notes::C.raise(2), Notes::D);
    assert_eq!(Notes::C.raise(3), Notes::DSharp);
    assert_eq!(Notes::C.raise(4), Notes::E);
    assert_eq!(Notes::C.raise(5), Notes::F);
    assert_eq!(Notes::C.raise(6), Notes::FSharp);
    assert_eq!(Notes::C.raise(7), Notes::G);
    assert_eq!(Notes::C.raise(8), Notes::GSharp);
    assert_eq!(Notes::C.raise(9), Notes::A);
    assert_eq!(Notes::C.raise(10), Notes::ASharp);
    assert_eq!(Notes::C.raise(11), Notes::B);
    assert_eq!(Notes::C.raise(12), Notes::C);
    assert_eq!(Notes::C.raise(13), Notes::CSharp);
}

#[test]
fn lower() {
    assert_eq!(Notes::C.lower(0), Notes::C);
    assert_eq!(Notes::C.lower(1), Notes::B);
    assert_eq!(Notes::C.lower(2), Notes::ASharp);
    assert_eq!(Notes::C.lower(3), Notes::A);
    assert_eq!(Notes::C.lower(4), Notes::GSharp);
    assert_eq!(Notes::C.lower(5), Notes::G);
    assert_eq!(Notes::C.lower(6), Notes::FSharp);
    assert_eq!(Notes::C.lower(7), Notes::F);
    assert_eq!(Notes::C.lower(8), Notes::E);
    assert_eq!(Notes::C.lower(9), Notes::DSharp);
    assert_eq!(Notes::C.lower(10), Notes::D);
    assert_eq!(Notes::C.lower(11), Notes::CSharp);
    assert_eq!(Notes::C.lower(12), Notes::C);
    assert_eq!(Notes::C.lower(13), Notes::B);
}

#[test]
fn shift() {
    assert_eq!(Notes::C.shift(0), Notes::C);
    assert_eq!(Notes::C.shift(5), Notes::F);
    assert_eq!(Notes::C.shift(-5), Notes::G);
}

#[test]
fn shift_by_interval() {
    assert_eq!(Notes::C.shift_by_interval(Interval::perfect_fifth()), Notes::G);
}
