pub mod prefix;
mod unit_impls;

use std::marker::PhantomData;
use crate::prefix::Prefix;

pub trait UnitPrefix {}

pub trait Unit<P: UnitPrefix, T> {
    fn new(value: T) -> Self;
}

#[derive(Debug)]
pub struct Mass<P: UnitPrefix, T> {
    value: T,
    prefix: Prefix,
    phantom: PhantomData<P>
}

#[derive(Debug)]
pub struct Time<P: UnitPrefix, T> {
    value: T,
    prefix: Prefix,
    phantom: PhantomData<P>
}

#[derive(Debug)]
pub struct AmountOfSubstance<P: UnitPrefix, T> {
    value: T,
    prefix: Prefix,
    phantom: PhantomData<P>
}

#[derive(Debug)]
pub struct ElectricCurrent<P: UnitPrefix, T> {
    value: T,
    prefix: Prefix,
    phantom: PhantomData<P>
}

#[derive(Debug)]
pub struct Temperature<P: UnitPrefix, T> {
    value: T,
    prefix: Prefix,
    phantom: PhantomData<P>
}

#[derive(Debug)]
pub struct LuminousIntensity<P: UnitPrefix, T> {
    value: T,
    prefix: Prefix,
    phantom: PhantomData<P>
}

mod tests {
    use crate::{Mass, Unit};
    use crate::prefix::{Exa, Peta};

    #[test]
    pub fn test() {
        let x= Mass::<Exa, f32>::new(10.0);
        let y= Mass::<Peta, f32>::new(10.0);
        // // let z: Mass<Dynamic, f32> = Mass::new(10.0);
        println!("{:?}", x);
        println!("{:?}", y);
    }
}