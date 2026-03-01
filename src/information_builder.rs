use std::io::{self, Write};

use num_bigint::BigUint;

use crate::{
    bit_builder::{self, BitBuilder},
    frac::Frac,
};

pub struct InformationBuilder {
    loc: Frac,
    precision: Frac,
}

impl InformationBuilder {
    pub fn new() -> Self {
        Self {
            loc: Frac::zero(),
            precision: Frac::one(),
        }
    }
    //Max is exclusive
    pub fn write(&mut self, max: BigUint, pick: BigUint) {
        assert!(pick < max);
        let inv_max = Frac::inverse(max);
        self.precision = &self.precision * &inv_max;
        self.loc = &self.loc + &(&self.precision * &pick)
    }

    pub fn write_distr(&mut self, distr: &[Frac], pick: usize) {
        assert!(pick < distr.len());

        let accum: Frac = distr
            .iter()
            .take(pick)
            .fold(Frac::zero(), |accum, x| &accum + x);
        self.loc = &self.loc + &(&self.precision * &accum);
        self.precision = &self.precision * &distr[pick];
    }

    pub fn write_to_stream(&self, stream: &mut impl Write) -> Result<(), io::Error> {
        //println!("Loc: {:?}\nPerc: {:?}", self.loc, self.precision);

        let mut bit_builder = BitBuilder::new(stream);
        let mut loc = Frac::zero();
        let mut precision = Frac::inverse(BigUint::from(2usize));
        let upper_bound = &self.loc + &self.precision;

        while loc < self.loc {
            if &loc + &precision >= upper_bound {
                //We can't overshoot because we won't be able to go back
                //println!("Wrote 0");
                bit_builder.write_bit(false)?;
            } else {
                //println!("Wrote 1");
                bit_builder.write_bit(true)?;
                loc = &loc + &precision;
            }
            precision = &precision / &BigUint::from(2usize);
        }
        bit_builder.flush()
    }
}
