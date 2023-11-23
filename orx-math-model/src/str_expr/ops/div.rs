use super::mul::wrap_if_needed;

pub(crate) fn div(lhs: &str, rhs: &str, result: &mut String) {
    match (lhs, rhs) {
        ("0", _) => super::mul::mul(lhs, rhs, result),
        (_, "0") => panic!("divided by zero"),
        (_, "1") => super::mul::mul(lhs, rhs, result),
        (_, "-1") => super::mul::mul(lhs, rhs, result),
        _ => {
            wrap_if_needed(lhs, result);
            result.push_str(" / ");
            wrap_if_needed(rhs, result);
        }
    }
}
