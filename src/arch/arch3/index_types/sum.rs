use alloc::string::String;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IdxInVariant<'d> {
    Usize(usize),
    Str(&'d String),
    I64(i64),
}
