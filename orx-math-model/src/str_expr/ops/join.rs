pub(crate) fn join_terms<I, S>(mut keys: I, result: &mut String)
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    if let Some(first) = keys.next() {
        result.push_str(first.as_ref());

        for next in keys {
            let is_negative = is_negative(next.as_ref());

            if is_negative {
                result.push_str(" - ");
                super::neg::neg(next.as_ref(), result);
            } else {
                result.push_str(" + ");
                result.push_str(next.as_ref());
            }
        }
    }
}

pub(crate) fn is_negative(key: &str) -> bool {
    key.chars().next().map(|c| c == '-').unwrap_or(false)
}
