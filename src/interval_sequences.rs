use rug::Rational;
use crate::Note;
use crate::just_intervals::JustInterval;

#[derive(Debug, PartialEq)]
pub struct IntervalSequence {
    pub intervals: Vec<JustInterval>,
    pub half_steps: i32,
    pub freq_scale: Rational,    
}

impl IntervalSequence {
    pub fn new() -> Self {
        IntervalSequence {
            intervals: Vec::<JustInterval>::new(),
            half_steps: 0,
            freq_scale: Rational::from((1,1))
        }
    }

    pub fn add_interval(&mut self, interval: JustInterval) {
        self.freq_scale *= interval.get_freq_scale();
        self.half_steps += interval.get_half_steps();
        self.intervals.push(interval);
    }

    pub fn get_half_steps(&self) -> i32 {
        self.half_steps
    }

    pub fn get_freq_scale(&self) -> Rational {
        self.freq_scale.clone()
    }

    pub fn to_notes(&self, startingnote: Note) -> Vec<Note> {
        let mut note_sequence = Vec::new();
        note_sequence.push(startingnote);
        let mut last_note = startingnote;

        for iinterval in self.intervals.iter() {
            let shifted_note = last_note.shift_by_interval(iinterval.clone());
            last_note = shifted_note;
            note_sequence.push(shifted_note);
        }
        note_sequence
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_interval() {
        let mut seq = IntervalSequence::new();
        seq.add_interval(JustInterval::MajorThird);
        let ref_seq = IntervalSequence {
            intervals: vec![JustInterval::MajorThird],
            half_steps: 4,
            freq_scale: Rational::from((5,4))
        };
        assert_eq!(seq, ref_seq);

        let mut seq = IntervalSequence::new();
        seq.add_interval(JustInterval::MajorThird);
        seq.add_interval(JustInterval::PerfectFourth);
        seq.add_interval(JustInterval::PerfectFourth);
        seq.add_interval(JustInterval::IPerfectFifth);
        seq.add_interval(JustInterval::IPerfectFifth);
        let ref_seq = IntervalSequence {
            intervals: vec![JustInterval::MajorThird,
                            JustInterval::PerfectFourth,
                            JustInterval::PerfectFourth,
                            JustInterval::IPerfectFifth,
                            JustInterval::IPerfectFifth],
            half_steps: 0,
            freq_scale: Rational::from((80,81))
        };
        assert_eq!(seq, ref_seq);
    }

    #[test]
    fn to_note_squence() {
        let startingnote = Note::new("C", 3);
        let mut sequence = IntervalSequence::new();
        sequence.add_interval(JustInterval::MajorThird);
        sequence.add_interval(JustInterval::IPerfectFourth);
        sequence.add_interval(JustInterval::PerfectFifth);
    
        let note_sequence = sequence.to_notes(startingnote);
        assert_eq!(note_sequence, vec![Note::new("C", 3), Note::new("E", 3), Note::new("B", 2), Note::new("F#", 3)]);
    }
}
