const PARS_OPEN: [char; 3] = ['(', '{', '['];
const PARS_CLOSE: [char; 3] = [')', '}', ']'];

pub(crate) fn neg(key: &str, result: &mut String) {
    let mut chars = key.chars();
    match chars.next() {
        None => result.push('0'),
        Some(first) => {
            if first != '-' {
                result.push('-');
                result.push(first);
            }

            let mut num_open_pars = 0;

            for c in chars {
                if PARS_OPEN.contains(&c) {
                    num_open_pars += 1;
                    result.push(c);
                } else if PARS_CLOSE.contains(&c) {
                    num_open_pars -= 1;
                    result.push(c);
                } else if num_open_pars == 0 {
                    match c {
                        '+' => result.push('-'),
                        '-' => result.push('+'),
                        _ => result.push(c),
                    }
                } else {
                    result.push(c);
                }
            }
        }
    }
}
