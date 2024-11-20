pub(crate) fn mul(lhs: &str, rhs: &str, result: &mut String) {
    match (lhs, rhs) {
        ("0", _) => result.push('0'),
        (_, "0") => result.push('0'),
        ("1", _) => result.push_str(rhs),
        (_, "1") => result.push_str(lhs),

        ("-1", _) => super::neg::neg(rhs, result),
        (_, "-1") => super::neg::neg(lhs, result),

        _ => {
            wrap_if_needed(lhs, result);
            result.push_str(" * ");
            wrap_if_needed(rhs, result);
        }
    }
}

pub(crate) fn wrap_if_needed(str: &str, result: &mut String) {
    if is_wrap_needed(str) {
        result.push('(');
        result.push_str(str);
        result.push(')');
    } else {
        result.push_str(str);
    }
}

fn is_wrap_needed(str: &str) -> bool {
    let mut num_open_pars = 0;
    let mut first = true;
    for c in str.chars() {
        match c {
            '(' => num_open_pars += 1,
            ')' => num_open_pars -= 1,
            _ => {
                if num_open_pars == 0 && (c == '+' || c == '-' && !first) {
                    return true;
                }
            }
        }
        first = false;
    }
    false
}
