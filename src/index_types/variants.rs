#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IdxInVariant<'d> {
    Usize(usize),
    Str(&'d str),
    I64(i64),
}
