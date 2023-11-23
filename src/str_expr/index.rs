const PARENTHESES: (char, char) = ('(', ')');

pub fn index<const D: usize>(key: &str, indices: [&str; D]) -> String {
    let mut copy_key = String::with_capacity(0);

    copy_key.push_str(key);
    copy_key.push(PARENTHESES.0);

    let mut first = true;
    for x in indices {
        if !first {
            copy_key.push_str(", ");
        }
        first = false;
        copy_key.push_str(x);
    }
    copy_key.push(PARENTHESES.1);
    copy_key
}
