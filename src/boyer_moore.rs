use std::collections::HashMap;

fn make_last_occurrences_table(pattern: &String) -> HashMap<char, usize> {
    let mut l = HashMap::new();

    for (i, c) in pattern.chars().enumerate() {
        l.insert(c, i);
    }

    l
}

fn make_z_array(s: &String) -> Vec<usize> {
    let chars: Vec<char> = s.chars().collect();
    let n = s.len();
    let mut z = vec![0; n];

    let mut l = 0;
    let mut r = 0;

    for i in 1..n {
        if i < r {
            z[i] = std::cmp::min(r - i, z[i - l]);
        }
        while i + z[i] < n && chars[z[i]] == chars[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }

    return z;
}

fn make_good_suffix_table(pattern: &String) -> Vec<i32> {
    let chars: Vec<char> = pattern.chars().collect();
    let mut table = vec![-1; chars.len()];

    let mut z = make_z_array(&chars.iter().rev().collect::<String>());
    z.reverse();

    for j in 0..chars.len() - 1 {
        let i = chars.len() - z[j];
        if i != chars.len() {
            table[i] = j as i32;
        }
    }

    table
}

fn make_full_shift_table(pattern: &String) -> Vec<i32> {
    let chars: Vec<char> = pattern.chars().collect();
    let mut table = vec![0; chars.len()];

    let z = make_z_array(pattern);
    let mut longest = 0;
    for i in (0..chars.len()).rev() {
        // is a suffix
        if z[i] == pattern.len() - i {
            longest = std::cmp::max(longest, z[i] as i32);
        }
        table[i] = longest;
    }

    table
}

pub fn boyer_moore(text: String, pattern: String) -> Vec<usize> {
    let mut matches = vec![];

    if pattern.len() == 0 || pattern.len() > text.len() {
        return matches;
    }

    let last_occ_table = make_last_occurrences_table(&pattern);
    let good_suffix_table: Vec<i32> = make_good_suffix_table(&pattern);
    let full_shift_table: Vec<i32> = make_full_shift_table(&pattern);

    let text: Vec<char> = text.chars().collect();
    let pattern: Vec<char> = pattern.chars().collect();

    let mut pattern_idx: i32 = (pattern.len() - 1) as i32;
    // start of cur window
    let mut text_idx = 0;
    let mut previous_text_end_idx = -1;

    while text_idx + (pattern.len() - 1) < text.len() {
        if pattern_idx < 0 || previous_text_end_idx == text_idx as i32 + pattern_idx {
            matches.push(text_idx);
            pattern_idx = (pattern.len() - 1) as i32;
            let skip = pattern.len() as i32 - full_shift_table[0];
            text_idx += skip as usize;
        } else if text_idx as i32 + pattern_idx > previous_text_end_idx
            && text[text_idx + pattern_idx as usize] == pattern[pattern_idx as usize]
        {
            pattern_idx -= 1;
        } else {
            let bad_skip: i32;
            if let Some(occ_idx) = last_occ_table.get(&text[text_idx + pattern_idx as usize]) {
                if *occ_idx >= (pattern_idx as usize) {
                    bad_skip = 1;
                } else {
                    bad_skip = pattern_idx - *occ_idx as i32;
                }
            } else {
                bad_skip = pattern.len() as i32;
            };

            let good_skip: i32;
            if pattern_idx + 1 == pattern.len() as i32 {
                good_skip = 1;
            } else if good_suffix_table[pattern_idx as usize + 1] == -1 {
                good_skip = pattern.len() as i32 - full_shift_table[pattern_idx as usize + 1];
            } else {
                good_skip = pattern.len() as i32 - 1 - good_suffix_table[pattern_idx as usize + 1];
            };

            let skip = std::cmp::max(std::cmp::max(bad_skip, good_skip), 1);
            // i + 1 is last char matching in present window. if we skip more than that then
            // the new window's start is in between i + 1 and the last windows end. We know this
            // part is matching. We skipped an aligned to a prefix (via one of the heuristics), 
            // and we know this prefix is already matching so no need to check this part again
            if skip >= pattern_idx + 1 {
                previous_text_end_idx = (text_idx + pattern.len() - 1) as i32;
            }

            text_idx += skip as usize;
            pattern_idx = (pattern.len() - 1) as i32;
        }
    }

    matches
}
