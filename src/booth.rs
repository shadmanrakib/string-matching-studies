// https://en.wikipedia.org/wiki/Lexicographically_minimal_string_rotation#:~:text=Booth's%20Algorithm,-An%20efficient%20algorithm&text=The%20algorithm%20uses%20a%20modified,once%20as%20they%20wrap%20around.
/*
def least_rotation(s: str) -> int:
    """Booth's lexicographically minimal string rotation algorithm."""
    n = len(s)
    f = [-1] * (2 * n)
    k = 0
    for j in range(1, 2 * n):
        i = f[j - k - 1]
        while i != -1 and s[j % n] != s[(k + i + 1) % n]:
            if s[j % n] < s[(k + i + 1) % n]:
                k = j - i - 1
            i = f[i]
        if i == -1 and s[j % n] != s[(k + i + 1) % n]:
            if s[j % n] < s[(k + i + 1) % n]:
                k = j
            f[j - k] = -1
        else:
            f[j - k] = i + 1
    return k
 */

pub fn booth(s: String) -> String {
    let n = s.len();

    if n == 0 {
        return "".to_string();
    }

    let s = s.chars().collect::<Vec<char>>();
    let mut least_rotation_idx = 0;

    let mut partial_match = vec![-1_i32; 2 * n];

    for i in 1..2 * n {
        // this essentially does KMP but always makes sure to align the partial match
        // to the start, overwriting previous match values
        let mut prev_prefix_len = partial_match[i - least_rotation_idx - 1];

        // backtracking to find largest prefix matching if possible
        // making sure to update the rotation if we discover a smaller
        // lexicographic shift by checking the diff char
        loop {
            // cur char, making sure to wrap around
            let cur_char = s[i % n];
            // char we are trying to match
            let to_match_char = s[(least_rotation_idx as i32 + prev_prefix_len + 1) as usize % n];

            if prev_prefix_len == -1 || cur_char == to_match_char {
                break;
            }

            if cur_char < to_match_char {
                let prev_idx = i - 1;
                let pattern_start = prev_idx - prev_prefix_len as usize;
                least_rotation_idx = pattern_start;
            }

            prev_prefix_len = partial_match[prev_prefix_len as usize];
        }

        // cur char, making sure to wrap around
        let cur_char = s[i % n];
        // char we are trying to match
        let to_match_char = s[(least_rotation_idx as i32 + prev_prefix_len + 1) as usize % n];

        if prev_prefix_len == -1 && cur_char != to_match_char {
            // no matching prefix

            // if we find a smaller start, use it
            if cur_char < to_match_char {
                least_rotation_idx = i;
            }
            // no match found so update the right slot
            // given the least_rotation_idx associated value is at idx 0
            partial_match[i - least_rotation_idx] = -1;
        } else {
            // update to new prefix len
            // includes when we detect a prefix of len 1
            partial_match[i - least_rotation_idx] = prev_prefix_len + 1;
        }
    }

    let wrapped = s[0..least_rotation_idx].iter().collect::<String>();
    let prefix = s[least_rotation_idx..n].iter().collect::<String>();

    prefix + &wrapped
}
