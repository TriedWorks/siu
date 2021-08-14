use crate::prefix::*;
use crate::{Unit, Mass, Time, AmountOfSubstance, ElectricCurrent, Temperature, LuminousIntensity};

macro_rules! impl_units {
    ($($prefix:ident => [$($unit:ident),*]),* $(,)?) => {
        $(
            $(
                impl<T> Unit<$prefix, T> for $unit<$prefix, T> {
                    fn new(value: T) -> Self {
                        Self {
                            value,
                            prefix: Prefix::$prefix,
                            phantom: Default::default()
                        }
                    }
                }
            )*
        )*
    }
}

impl_units! {
    Exa   => [Mass, Time, AmountOfSubstance, ElectricCurrent, Temperature, LuminousIntensity],
    Peta  => [Mass, Time, AmountOfSubstance, ElectricCurrent, Temperature, LuminousIntensity],
    Tera  => [Mass, Time, AmountOfSubstance, ElectricCurrent, Temperature, LuminousIntensity],
    Giga  => [Mass, Time, AmountOfSubstance, ElectricCurrent, Temperature, LuminousIntensity],
    Mega  => [Mass, Time, AmountOfSubstance, ElectricCurrent, Temperature, LuminousIntensity],
    Kilo  => [Mass, Time, AmountOfSubstance, ElectricCurrent, Temperature, LuminousIntensity],
    Hecto => [Mass, Time, AmountOfSubstance, ElectricCurrent, Temperature, LuminousIntensity],
    Deka  => [Mass, Time, AmountOfSubstance, ElectricCurrent, Temperature, LuminousIntensity],
    None  => [Mass, Time, AmountOfSubstance, ElectricCurrent, Temperature, LuminousIntensity],
    Deci  => [Mass, Time, AmountOfSubstance, ElectricCurrent, Temperature, LuminousIntensity],
    Centi => [Mass, Time, AmountOfSubstance, ElectricCurrent, Temperature, LuminousIntensity],
    Milli => [Mass, Time, AmountOfSubstance, ElectricCurrent, Temperature, LuminousIntensity],
    Micro => [Mass, Time, AmountOfSubstance, ElectricCurrent, Temperature, LuminousIntensity],
    Nano  => [Mass, Time, AmountOfSubstance, ElectricCurrent, Temperature, LuminousIntensity],
    Pico  => [Mass, Time, AmountOfSubstance, ElectricCurrent, Temperature, LuminousIntensity],
    Femto => [Mass, Time, AmountOfSubstance, ElectricCurrent, Temperature, LuminousIntensity],
    Atto  => [Mass, Time, AmountOfSubstance, ElectricCurrent, Temperature, LuminousIntensity],
}


// impl<T> Unit<Milli, T> for Mass<Milli, T> {
//     fn new(value: T) -> Self {
//         Self {
//             value,
//             prefix: Prefix::Milli,
//             phantom: Default::default()
//         }
//     }
// }
//
// impl<T> Unit<Dynamic, T> for Mass<Dynamic, T>  {
//     fn new(value: T) -> Self {
//         Self {
//             value,
//             prefix: Prefix::Unit,
//             phantom: Default::default()
//         }
//     }
// }