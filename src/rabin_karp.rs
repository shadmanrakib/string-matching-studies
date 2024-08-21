fn char_hash(c: char) -> u32 {
    u32::from(c) + 1 % 512
}

pub fn rabin_karp(text: String, pattern: String) -> Vec<usize> {
    let mut matches = vec![];

    if pattern.len() == 0 || pattern.len() > text.len() {
        return matches;
    }

    let text = text.chars().collect::<Vec<char>>();
    let pattern = pattern.chars().collect::<Vec<char>>();

    let n = text.len();
    let m = pattern.len();

    let mut target_hash = 0;
    let mut hash: u32 = 0;
    for i in 0..m {
        target_hash += char_hash(pattern[i]);
        hash += char_hash(text[i])
    }

    for exiting in 0..(n - m + 1) {
        if target_hash == hash {
            let mut equal = true;
            for i in 0..m {
                if text[exiting + i] != pattern[i] {
                    equal = false;
                    break;
                }
            }
            if equal {
                matches.push(exiting);
            }
        }

        // remove exiting add entering
        let entering = exiting + m;
        if entering < n {
            hash -= char_hash(text[exiting]);
            hash += char_hash(text[entering]);
        }
    }

    matches
}
