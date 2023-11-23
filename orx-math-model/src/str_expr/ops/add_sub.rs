use crate::numerics::ops::Ops;

pub fn add(lhs_key: &str, rhs_key: &str, result: &mut String) {
    add_sub(Ops::Add, lhs_key, rhs_key, result);
}
pub fn sub(lhs_key: &str, rhs_key: &str, result: &mut String) {
    add_sub(Ops::Sub, lhs_key, rhs_key, result);
}

fn add_sub(ops: Ops, lhs_key: &str, rhs_key: &str, result: &mut String) {
    if rhs_key == "0" {
        result.push_str(lhs_key);
    } else if lhs_key == "0" {
        if matches!(ops, Ops::Sub) {
            super::neg::neg(rhs_key, result);
        } else {
            result.push_str(rhs_key);
        }
    } else {
        let (rhs_key, is_rhs_neg) = if let Some(stripped) = rhs_key.strip_prefix('-') {
            (stripped, true)
        } else {
            (rhs_key, false)
        };
        let ops_char = match (ops, is_rhs_neg) {
            (Ops::Add, false) => '+',
            (Ops::Add, true) => '-',
            (Ops::Sub, false) => '-',
            (Ops::Sub, true) => '+',
            _ => panic!("nono"),
        };

        result.push_str(lhs_key);
        result.push(' ');
        result.push(ops_char);
        result.push(' ');
        result.push_str(rhs_key);
    }
}
