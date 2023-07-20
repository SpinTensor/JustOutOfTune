use std::ops::{Neg, Add, Sub};
use num_traits::Inv;
use rug::Rational;

#[derive(Debug, PartialEq, Clone)]
pub struct Interval {
    pub half_tone_steps: i8,
    pub frequency_scale: Rational,
}

impl Interval {
    pub fn unison() -> Self {
        Interval::new(0, (1, 1))
    }
    pub fn minor_second() -> Self {
        Interval::new(1, (16,15))
    }
    pub fn major_second() -> Self {
        Interval::new(2, (9, 8))
    }
    pub fn minor_third() -> Self {
        Interval::new(3, (6, 5))
    }
    pub fn major_third() -> Self {
        Interval::new(4, (5, 4))
    }
    pub fn perfect_fourth() -> Self {
        Interval::new(5, (4, 3))
    }
    pub fn perfect_fifth() -> Self {
        Interval::new(7, (3, 2))
    }
    pub fn minor_sixth() -> Self {
        Interval::new(8, (8, 5))
    }
    pub fn major_sixth() -> Self {
        Interval::new(9, (5, 3))
    }
    pub fn minor_seventh() -> Self {
        Interval::new(10, (16, 9))
    }
    pub fn major_seventh() -> Self {
        Interval::new(11, (15, 8))
    }
    pub fn perfect_octave() -> Self {
        Interval::new(12, (2, 1))
    }
}

impl Interval {
    pub fn new(half_tone_steps: i8, frequency_scale: (u8, u8)) -> Interval {
        Interval {
            half_tone_steps,
            frequency_scale: Rational::from(frequency_scale)
        }
    }
}

impl Neg for Interval {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            half_tone_steps: -self.half_tone_steps,
            frequency_scale: self.frequency_scale.inv()
        }
    }
}
#[test]
fn negate() {
    let interval = Interval::new(1, (2,3));
    let neginterval = interval.clone().neg();
    assert_eq!(neginterval, Interval::new(-1, (3,2)));
}

impl Add for Interval {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            half_tone_steps: self.half_tone_steps + other.half_tone_steps,
            frequency_scale: self.frequency_scale * other.frequency_scale
        }
    }
}
#[test]
fn addition() {
    let interval_a = Interval::major_third();
    let interval_b = Interval::perfect_fifth();
    let interval_c = interval_a + interval_b;
    assert_eq!(interval_c, Interval::new(11, (15, 8)));
}

impl Sub for Interval {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            half_tone_steps: self.half_tone_steps - other.half_tone_steps,
            frequency_scale: self.frequency_scale / other.frequency_scale
        }
    }
}
#[test]
fn subtraction() {
    let interval_a = Interval::new(11, (15, 8));
    let interval_b = Interval::perfect_fifth();
    let interval_c = interval_a - interval_b;
    assert_eq!(interval_c, Interval::major_third());
}
