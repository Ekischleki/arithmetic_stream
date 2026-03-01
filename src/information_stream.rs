use std::io::Read;

use num_bigint::BigUint;

use crate::{bitstream::Bitstream, frac::Frac};

pub struct InformationStream<S: Read> {
    bitstream: Bitstream<S>,
    loc: Frac,
    bit_weight: Frac,
}

impl<S: Read> InformationStream<S> {
    pub fn new(s: S) -> Self {
        Self {
            bitstream: Bitstream::new(s),
            loc: Frac::zero(),
            bit_weight: Frac::inverse(BigUint::from(2usize)),
        }
    }
    //Reads a bit from the bitstream and updates the location accordingly
    fn update_loc(&mut self) {
        let weighted_bit = &if self.bitstream.get_next_bit().unwrap_or(false) {
            println!("Read 1");
            Frac::one()
        } else {
            println!("Read 0");

            Frac::zero()
        } * &self.bit_weight;
        self.bit_weight = &self.bit_weight / &BigUint::from(2usize);
        self.loc = &self.loc + &weighted_bit;
    }
    pub fn get_int(&mut self, max: BigUint) -> BigUint {
        let precision = Frac::inverse(max.clone());
        //println!("Prec: {:?}", precision);
        while self.bit_weight > precision {
            //println!("Bit weight: {:?}", self.bit_weight);
            //As long as this is the case we certainly pass a boundary
            self.update_loc();
        }
        let res = loop {
            //Lower bound for loc if the rest of the bitstream is infinite zeroes
            let k1 = Self::get_inside_loc(&self.loc, &precision);
            //Upper bound for loc if the rest of the bitstream is infinite ones
            let k2 = Self::get_inside_loc(
                &(&self.loc + &(&self.bit_weight * &BigUint::from(2usize))),
                &precision,
            );

            //println!("loc: {:?}, k1: {:?}, k2: {:?}", self.loc, k1, k2);

            //If both upper and lower bounds lie within the same value, we can be certain that we don't need anymore bits and can go on with rescaling.
            if &k1 == &k2 {
                break k1;
            }
            //We need more precision
            self.update_loc();
        };

        //Rescale bounds by max

        self.bit_weight = &self.bit_weight * &max;
        self.loc = &(&self.loc - &(&precision * &res)) * &max;

        return res;
    }
    //Loc should be between 0 and 1. Will give an int k such that loc = k * precision + c with 0 <= c < precision
    fn get_inside_loc(loc: &Frac, precision: &Frac) -> BigUint {
        let (l, p) = loc.make_comparable(precision);
        return l / p;
    }
}
