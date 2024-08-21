// Based on https://en.wikipedia.org/wiki/Knuth%E2%80%93Morris%E2%80%93Pratt_algorithm

fn kmp_partial_match_table(str: &String) -> Vec<i32> {
    let mut table = vec![0_i32; str.len() + 1];

    let chars: Vec<char> = str.chars().collect();
    let mut cnd: i32 = 0;

    table[0] = -1;
    for pos in 1..chars.len() {
        if chars[pos] == chars[cnd as usize] {
            table[pos] = table[cnd as usize];
        } else {
            table[pos] = cnd;
            while cnd >= 0 && chars[pos] != chars[cnd as usize] {
                cnd = table[cnd as usize];
            }
        }
        cnd += 1;
    }
    
    let last_idx = table.len() - 1;
    table[last_idx] = cnd;

    table
}

pub fn kmp(text: String, pattern: String) -> Vec<usize> {
    let mut matches = vec![];
    
    if pattern.len() == 0 || pattern.len() > text.len() {
        return matches;
    }

    let table = kmp_partial_match_table(&pattern);

    let text: Vec<char> = text.chars().collect();
    let pattern: Vec<char> = pattern.chars().collect();

    let mut text_idx = 0;
    let mut pattern_idx = 0;

    while text_idx < text.len() {
        if text[text_idx] == pattern[pattern_idx] {
            text_idx += 1;
            pattern_idx += 1;

            if pattern_idx == pattern.len() {
                let start_idx = text_idx - pattern_idx; 
                matches.push(start_idx);
                pattern_idx = table[pattern_idx] as usize;
            }
        } else if table[pattern_idx] < 0 {
            pattern_idx = 0;
            text_idx += 1;
        } else {
            pattern_idx = table[pattern_idx] as usize;
        }
    }

    matches
}