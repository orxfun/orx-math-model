use crate::draft::set::Set0;

pub trait Par {
    type Value;
}

pub trait Par0: Par {}

pub trait Par1: Par {
    type S0: Set0;
}
