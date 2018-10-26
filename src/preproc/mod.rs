///
/// Removes comments, indentation, and blank lines from the supplied
/// buffer. [DEPRECATED]
///
pub fn strip(s: String) -> String {
    let mut ret = String::new();
    'outer: for line in s.lines() {
        let mut words: Vec<_> = line.split(|c: char| c.is_whitespace() || c == ',').collect();

        // Remove blank strings inserted by `split`
        while words.len() > 0 && words[0] == "" {
            words.remove(0);
        }

        // Skip if blank line or comment
        if words.len() == 0 || words[0] == ";" {
            continue;
        }

        // Join lines with only a label in them
        if words.len() == 1 && words[0].contains(":") {
            ret.push_str(&words[0]);
            ret.push(' ');
            continue;
        }

        for word in words {
            match word.chars().next() {
                Some(';') => {
                    ret.push('\n');
                    continue 'outer;
                },
                None => continue,
                _ => {}
            }

            ret.push_str(&word);
            ret.push(' ');
        }

        ret.push('\n');
    }
    ret.pop();
    return ret;
}
