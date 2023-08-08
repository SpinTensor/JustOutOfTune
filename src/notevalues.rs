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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
