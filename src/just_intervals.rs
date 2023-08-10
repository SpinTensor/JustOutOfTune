use num_traits::{Inv};
use rug::Rational;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum JustInterval {
    Unison,
    MajorThird,
    IMajorThird,
    PerfectFourth,
    IPerfectFourth,
    PerfectFifth,
    IPerfectFifth,
}

impl JustInterval {
    pub fn get_freq_scale(&self) -> Rational {
        match self {
            JustInterval::Unison => Rational::from((1,1)),
            JustInterval::MajorThird => Rational::from((5,4)),
            JustInterval::IMajorThird => JustInterval::MajorThird.get_freq_scale().inv(),
            JustInterval::PerfectFourth => Rational::from((4,3)),
            JustInterval::IPerfectFourth => JustInterval::PerfectFourth.get_freq_scale().inv(),
            JustInterval::PerfectFifth => Rational::from((3,2)),
            JustInterval::IPerfectFifth => JustInterval::PerfectFifth.get_freq_scale().inv()
        }
    }
    pub fn get_half_steps(&self) -> i32 {
        match self {
            JustInterval::Unison => 0,
            JustInterval::MajorThird => 4,
            JustInterval::IMajorThird => -JustInterval::MajorThird.get_half_steps(),
            JustInterval::PerfectFourth => 5,
            JustInterval::IPerfectFourth => -JustInterval::PerfectFourth.get_half_steps(),
            JustInterval::PerfectFifth => 7,
            JustInterval::IPerfectFifth => -JustInterval::PerfectFifth.get_half_steps(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_freq_scale() {
        assert_eq!(JustInterval::MajorThird.get_freq_scale(),     Rational::from((5,4)));
        assert_eq!(JustInterval::IMajorThird.get_freq_scale(),    Rational::from((4,5)));
        assert_eq!(JustInterval::PerfectFourth.get_freq_scale(),  Rational::from((4,3)));
        assert_eq!(JustInterval::IPerfectFourth.get_freq_scale(), Rational::from((3,4)));
        assert_eq!(JustInterval::PerfectFifth.get_freq_scale(),   Rational::from((3,2)));
        assert_eq!(JustInterval::IPerfectFifth.get_freq_scale(),  Rational::from((2,3)));
    }

    #[test]
    fn get_half_steps() {
        assert_eq!(JustInterval::MajorThird.get_half_steps(),      4);
        assert_eq!(JustInterval::IMajorThird.get_half_steps(),    -4);
        assert_eq!(JustInterval::PerfectFourth.get_half_steps(),   5);
        assert_eq!(JustInterval::IPerfectFourth.get_half_steps(), -5);
        assert_eq!(JustInterval::PerfectFifth.get_half_steps(),    7);
        assert_eq!(JustInterval::IPerfectFifth.get_half_steps(),  -7);
    }
}
