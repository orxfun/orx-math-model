use crate::numerics::ops::Ops;

mod add_sub;
mod div;
mod join;
mod mul;
mod neg;

pub(crate) fn binary_ops(ops: Ops, lhs_key: &str, rhs_key: &str) -> String {
    let mut result = String::with_capacity(lhs_key.len() + rhs_key.len() + 8);
    match ops {
        Ops::Add => add_sub::add(lhs_key, rhs_key, &mut result),
        Ops::Sub => add_sub::sub(lhs_key, rhs_key, &mut result),
        Ops::Mul => mul::mul(lhs_key, rhs_key, &mut result),
        Ops::Div => div::div(lhs_key, rhs_key, &mut result),
    }
    result
}

pub(crate) fn neg(key: &str) -> String {
    let mut result = String::with_capacity(key.len() + 1);
    neg::neg(key, &mut result);
    result
}

pub(crate) fn join_terms<I, S>(keys: I) -> String
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut result = String::new();
    join::join_terms(keys, &mut result);
    result
}
