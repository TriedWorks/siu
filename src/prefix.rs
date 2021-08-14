use crate::UnitPrefix;

#[repr(i8)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Prefix {
    Yotta = 24,
    Zetta = 21,
    Exa   = 18,
    Peta  = 15,
    Tera  = 12,
    Giga  = 9,
    Mega  = 6,
    Kilo  = 3,
    Hecto = 2,
    Deka  = 1,
    None  = 0,
    Deci  = -1,
    Centi = -2,
    Milli = -3,
    Micro = -6,
    Nano  = -9,
    Pico  = -12,
    Femto = -15,
    Atto  = -18,
    Zepto = -21,
    Yocto = -24,
}


#[derive(Debug)]
pub struct Exa;
#[derive(Debug)]
pub struct Peta;
#[derive(Debug)]
pub struct Tera;
#[derive(Debug)]
pub struct Giga;
#[derive(Debug)]
pub struct Mega;
#[derive(Debug)]
pub struct Kilo;
#[derive(Debug)]
pub struct Hecto;
#[derive(Debug)]
pub struct Deka;
#[derive(Debug)]
pub struct None;
#[derive(Debug)]
pub struct Deci;
#[derive(Debug)]
pub struct Centi;
#[derive(Debug)]
pub struct Milli;
#[derive(Debug)]
pub struct Micro;
#[derive(Debug)]
pub struct Nano;
#[derive(Debug)]
pub struct Pico;
#[derive(Debug)]
pub struct Femto;
#[derive(Debug)]
pub struct Atto;
#[derive(Debug)]
pub struct Dynamic;

impl UnitPrefix for Exa {}
impl UnitPrefix for Peta {}
impl UnitPrefix for Tera {}
impl UnitPrefix for Giga {}
impl UnitPrefix for Mega {}
impl UnitPrefix for Kilo {}
impl UnitPrefix for Hecto {}
impl UnitPrefix for Deka {}
impl UnitPrefix for None {}
impl UnitPrefix for Deci {}
impl UnitPrefix for Centi {}
impl UnitPrefix for Milli {}
impl UnitPrefix for Micro {}
impl UnitPrefix for Nano {}
impl UnitPrefix for Pico {}
impl UnitPrefix for Femto {}
impl UnitPrefix for Atto {}
impl UnitPrefix for Dynamic {}