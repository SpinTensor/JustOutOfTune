use num_traits::{Inv, };
use std::ops::{Neg,Mul};
use std::cmp::{Ordering};
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

impl Neg for JustInterval {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            JustInterval::Unison         => self,
            JustInterval::MajorThird     => JustInterval::IMajorThird,
            JustInterval::IMajorThird    => JustInterval::MajorThird,
            JustInterval::PerfectFourth  => JustInterval::IPerfectFourth,
            JustInterval::IPerfectFourth => JustInterval::PerfectFourth,
            JustInterval::PerfectFifth   => JustInterval::IPerfectFifth,
            JustInterval::IPerfectFifth  => JustInterval::PerfectFifth
        }
    }
}

impl Mul<i32> for JustInterval {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        match rhs.cmp(&0) {
            Ordering::Greater => self,
            Ordering::Equal   => JustInterval::Unison,
            Ordering::Less    => -self
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

    #[test]
    fn negate() {
        let options = [JustInterval::Unison,
                       JustInterval::MajorThird, JustInterval::IMajorThird,
                       JustInterval::PerfectFourth, JustInterval::IPerfectFourth,
                       JustInterval::PerfectFifth, JustInterval::IPerfectFifth];
        for value in options {
            let negated = -value;
            assert_eq!(value, -negated);
            assert_eq!(value.get_half_steps(), -(negated.get_half_steps()));
            assert_eq!(value.get_freq_scale(), (negated.get_freq_scale().inv()));
        }
    }

    #[test]
    fn multiply() {
        let options = [JustInterval::Unison,
                       JustInterval::MajorThird, JustInterval::IMajorThird,
                       JustInterval::PerfectFourth, JustInterval::IPerfectFourth,
                       JustInterval::PerfectFifth, JustInterval::IPerfectFifth];
        for value in options {
            for factor in 1..3 {
                assert_eq!(value*factor, value);
                assert_eq!(value*(-factor), -value);
            }
            for factor in [0] {
                assert_eq!(value*factor, JustInterval::Unison);
            }
        }
    }
}
