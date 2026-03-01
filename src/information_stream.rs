use std::io::Read;

use num_bigint::BigUint;

use crate::{bitstream::Bitstream, frac::Frac};

pub struct InformationStream<S: Read> {
    bitstream: Bitstream<S>,
    loc: Frac,
    bit_weight: Frac,
    stream_end: bool,
}

impl<S: Read> InformationStream<S> {
    pub fn new(s: S) -> Self {
        Self {
            bitstream: Bitstream::new(s),
            loc: Frac::zero(),
            bit_weight: Frac::inverse(BigUint::from(2usize)),
            stream_end: false,
        }
    }
    //Reads a bit from the bitstream and updates the location accordingly
    fn update_loc(&mut self) {
        let weighted_bit = &if self.bitstream.get_next_bit().unwrap_or_else(|_| {
            self.stream_end = true;
            false
        }) {
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
    pub fn get_from_distr(&mut self, distr: &[Frac]) -> usize {
        let mut size;
        let res = loop {
            let (k1, k2);
            (k1, k2, size) = Self::get_inside_distr(
                &self.loc,
                &(&self.loc + &(&self.bit_weight * &BigUint::from(2usize))),
                distr,
            );

            //If both upper and lower bounds lie within the same value, we can be certain that we don't need anymore bits and can go on with rescaling.
            if &k1 == &k2 {
                //println!("Size: {:?}\nmax: {:?}\nloc:{:?}", size, distr[k1], self.loc);
                break k1;
            }
            //We need more precision
            self.update_loc();
        };

        //Rescale bounds by max

        let mag = &distr[res];

        self.bit_weight = &self.bit_weight / mag;
        self.loc = &(&self.loc - &size) / mag;

        return res;
    }
    //Loc should be between 0 and 1. Will give an int k such that loc = k * precision + c with 0 <= c < precision
    fn get_inside_loc(loc: &Frac, precision: &Frac) -> BigUint {
        let (l, p) = loc.make_comparable(precision);
        return l / p;
    }
    fn get_inside_distr(loc: &Frac, upper: &Frac, distr: &[Frac]) -> (usize, usize, Frac) {
        let mut accum = Frac::zero();
        let mut distr = distr.into_iter().enumerate();
        let mut k1 = 0;
        let mut size = Frac::zero();

        while &accum < &loc {
            size = accum;
            accum = &size
                + if let Some((i, s)) = distr.next() {
                    k1 = i;
                    s
                } else {
                    return (distr.len(), distr.len(), size);
                }
        }
        let mut k2 = k1;
        while &accum < &upper {
            accum = &accum
                + if let Some((i, s)) = distr.next() {
                    k2 = i;
                    s
                } else {
                    return (k1, distr.len(), size);
                }
        }
        (k1, k2, size)
    }

    ///Decoding will still continue even if you've reached the stream end, this is because eof gets implicitly converted into 0
    pub fn stream_end(&self) -> bool {
        self.stream_end
    }
}
