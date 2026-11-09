use crate::ptr::ValPtr;

pub enum IdxInPtr<'d> {
    Usize(ValPtr<usize>),
    Str(ValPtr<&'d str>),
    I64(ValPtr<i64>),
}
