#![cfg_attr(not(test), no_std)]

use core::fmt;

pub struct Ratio {
    numerator: u32,
    denominator: u32,
    mult: u32,
    shift: u32,
}

impl Ratio {
    pub const fn new(numerator: u32, denominator: u32) -> Self {
        if numerator == 0 {
            return Self {
                numerator,
                denominator,
                mult: 0,
                shift: 0,
            };
        }

        let mut shift = 32;
        let mut mult;
        loop {
            mult = (((numerator as u64) << shift) + denominator as u64 / 2) / denominator as u64;
            if mult <= u32::MAX as u64 || shift == 0 {
                break;
            }
            shift -= 1;
        }

        if mult % 2 == 0 && shift > 0 {
            mult /= 2;
            shift -= 1;
        }

        Self {
            numerator,
            denominator,
            mult: mult as u32,
            shift,
        }
    }

    pub fn multiply(&self, v: u64) -> u64 {
        ((v as u128 * self.mult as u128) >> self.shift) as u64
    }
}

impl fmt::Debug for Ratio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Ratio: {}/{} ~= {}/(1 << {}) = {}>>{}",
            self.numerator, self.denominator, self.mult, self.shift, self.mult, self.shift
        )
    }
}

impl PartialEq<Ratio> for Ratio {
    fn eq(&self, other: &Ratio) -> bool {
        self.mult == other.mult && self.shift == other.shift
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ratio() {
        let a = Ratio::new(1, 3);
        let b = Ratio::new(2, 6);
        assert_eq!(a, b);
    }
}
