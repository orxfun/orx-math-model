use crate::symbols::Symbol;

#[derive(Clone, Copy)]
pub enum Count<'a> {
    Sym(Symbol<'a>),
    Const(usize),
}
