use alloc::string::String;

pub enum ValTyp {
    Usize,
    RefString,
    I64,
}

#[repr(C)]
pub union UVal<'d> {
    pub usize: usize,
    pub ref_string: &'d String,
    pub i64: i64,
}

pub struct Val<'d> {
    typ: ValTyp,
    data: UVal<'d>,
}
