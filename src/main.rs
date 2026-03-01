use crate::{
    frac::Frac, information_builder::InformationBuilder, information_stream::InformationStream,
};

pub mod bit_builder;
pub mod bitstream;
mod frac;
pub mod information_builder;
pub mod information_stream;

fn main() {
    let mut memory_stream: Vec<u8> = Vec::new();
    let mut information_builder = InformationBuilder::new();
    information_builder.write_distr(&[Frac::from_usize(1, 18), Frac::from_usize(17, 18)], 1);
    information_builder.write(3usize.into(), 2usize.into());
    //information_builder.write(85usize.into(), 0usize.into());

    information_builder
        .write_to_stream(&mut memory_stream)
        .unwrap();

    println!("{:?}", memory_stream);

    //let memory_stream = vec![72];

    let mut information_stream = InformationStream::new(memory_stream.as_slice());
    let res =
        information_stream.get_from_distr(&[Frac::from_usize(1, 18), Frac::from_usize(17, 18)]);
    println!("{}", res);
    let res = information_stream.get_int(3usize.into());
    println!("{}", res);
}
