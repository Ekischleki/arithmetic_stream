use std::{
    borrow::Cow,
    cmp,
    ops::{Add, Div, Mul, Sub},
};

use num_bigint::BigUint;

#[derive(Debug)]
pub struct Frac {
    numerator: BigUint,
    denominator: BigUint,
}

impl Mul for &Frac {
    type Output = Frac;

    fn mul(self, rhs: Self) -> Frac {
        let mut r = Frac {
            numerator: &self.numerator * &rhs.numerator,
            denominator: &self.denominator * &rhs.denominator,
        };
        r.simplify();
        r
    }
}

impl Mul<&BigUint> for &Frac {
    type Output = Frac;

    fn mul(self, rhs: &BigUint) -> Frac {
        let mut r = Frac {
            numerator: &self.numerator * rhs,
            denominator: self.denominator.clone(),
        };
        r.simplify();
        r
    }
}

impl Div for &Frac {
    type Output = Frac;

    fn div(self, rhs: Self) -> Frac {
        let mut r = Frac {
            numerator: &self.numerator * &rhs.denominator,
            denominator: &self.denominator * &rhs.numerator,
        };
        r.simplify();
        r
    }
}

impl Div<&BigUint> for &Frac {
    type Output = Frac;

    fn div(self, rhs: &BigUint) -> Frac {
        let mut r = Frac {
            numerator: self.numerator.to_owned(),
            denominator: &self.denominator * rhs,
        };
        r.simplify();
        r
    }
}

impl Add for &Frac {
    type Output = Frac;

    fn add(self, rhs: Self) -> Self::Output {
        //multiply a by denominator of b and b by denominator of a
        let mut r = Frac {
            denominator: &self.denominator * &rhs.denominator,
            numerator: &self.numerator * &rhs.denominator + &rhs.numerator * &self.denominator,
        };
        r.simplify();
        r
    }
}

impl Sub for &Frac {
    type Output = Frac;

    fn sub(self, rhs: Self) -> Self::Output {
        //multiply a by denominator of b and b by denominator of a
        let mut r = Frac {
            denominator: &self.denominator * &rhs.denominator,
            numerator: &self.numerator * &rhs.denominator - &rhs.numerator * &self.denominator,
        };
        r.simplify();
        r
    }
}

impl PartialEq for Frac {
    fn eq(&self, other: &Self) -> bool {
        &self.numerator * &other.denominator == &other.numerator * &self.denominator
    }
}

impl Eq for Frac {}

impl PartialOrd for Frac {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Frac {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        (&self.numerator * &other.denominator).cmp(&(&other.numerator * &self.denominator))
    }
}

impl Frac {
    pub fn make_comparable(&self, other: &Self) -> (BigUint, BigUint) {
        (
            &self.numerator * &other.denominator,
            &other.numerator * &self.denominator,
        )
    }
    pub fn inverse(x: BigUint) -> Self {
        Self {
            numerator: 1usize.into(),
            denominator: x,
        }
    }
    pub fn zero() -> Self {
        Self {
            numerator: 0usize.into(),
            denominator: 1usize.into(),
        }
    }
    pub fn one() -> Self {
        Self {
            numerator: 1usize.into(),
            denominator: 1usize.into(),
        }
    }
    fn gcd<'a>(mut a: Cow<'a, BigUint>, mut b: Cow<'a, BigUint>) -> BigUint {
        while b.as_ref() != &0usize.into() {
            let t = b.clone();
            b = Cow::Owned(a.as_ref() % b.as_ref());
            a = t;
        }
        a.into_owned()
    }
    pub fn simplify(&mut self) {
        if &self.numerator == &0usize.into() {
            *self = Self::zero();
            return;
        }
        let gcd = Self::gcd(
            Cow::Borrowed(&self.numerator),
            Cow::Borrowed(&self.denominator),
        );
        self.denominator /= &gcd;
        self.numerator /= &gcd;
    }
}
