#[derive(Clone, Copy, PartialEq)]
pub enum VarTyp {
    Continuous,
    Binary,
    Integer,
}

pub trait VarType {}

pub struct Continuous;
impl VarType for Continuous {}

pub struct Binary;
impl VarType for Binary {}

pub struct Integer;
impl VarType for Integer {}
