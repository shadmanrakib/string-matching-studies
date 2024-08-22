mod booth;
mod boyer_moore;
mod kmp;
mod rabin_karp;

fn main() {
    booth_tests();
    matching_tests();
}

fn booth_tests() {
    // lexicographically minimal rotations
    for (input, expected) in [("", ""), ("ppapple", "applepp"), ("aaaa", "aaaa"), ("aabbabsasba", "aaabbabsasb")] {
        assert_eq!(booth::booth(input.to_string()), expected.to_string());
    }
    println!("Booth tests passed");
}

fn matching_tests() {
    for (text, pattern) in [
        ("", ""),
        ("", "a"),
        ("", "foo"),
        ("fo", "foo"),
        ("foo", "foo"),
        ("oofofoofooo", "f"),
        ("oofofoofooo", "foo"),
        ("barfoobarfoo", "foo"),
        ("foo", ""),
        ("foo", "o"),
        ("abcABCabc", "A"),
        ("jrzm6jjhorimglljrea4w3rlgosts0w2gia17hno2td4qd1jz", "jz"),
        ("ekkuk5oft4eq0ocpacknhwouic1uua46unx12l37nioq9wbpnocqks6", "ks6"),
        ("999f2xmimunbuyew5vrkla9cpwhmxan8o98ec", "98ec"),
        ("9lpt9r98i04k8bz6c6dsrthb96bhi", "96bhi"),
        ("55u558eqfaod2r2gu42xxsu631xf0zobs5840vl", "5840vl"),
        ("", "a"),
        ("x", "a"),
        ("x", "x"),
        ("abc", "a"),
        ("abc", "b"),
        ("abc", "c"),
        ("abc", "x"),
        ("", "ab"),
        ("bc", "ab"),
        ("ab", "ab"),
        ("xab", "ab"),
        ("xab", "ab"),
        ("", "abc"),
        ("xbc", "abc"),
        ("abc", "abc"),
        ("xabc", "abc"),
        ("xabc", "abc"),
        ("xabxc", "abc"),
        ("", "abcd"),
        ("xbcd", "abcd"),
        ("abcd", "abcd"),
        ("xabcd", "abcd"),
        ("xyabcd", "abcd"),
        ("xbcqq", "abcqq"),
        ("abcqq", "abcqq"),
        ("xabcqq", "abcqq"),
        ("xyabcqq", "abcqq"),
        ("xabxcqq", "abcqq"),
        ("xabcqxq", "abcqq"),
        ("", "01234567"),
        ("32145678", "01234567"),
        ("01234567", "01234567"),
        ("x01234567", "01234567"),
        ("x0123456x01234567", "01234567"),
        ("xx01234567", "01234567"),
        ("", "0123456789"),
        ("3214567844", "0123456789"),
        ("0123456789", "0123456789"),
        ("x0123456789", "0123456789"),
        ("x012345678x0123456789", "0123456789"),
        ("xyz0123456789", "0123456789"),
        ("x01234567x89", "0123456789"),
        ("", "0123456789012345"),
        ("3214567889012345", "0123456789012345"),
        ("0123456789012345", "0123456789012345"),
        ("x0123456789012345", "0123456789012345"),
        ("x012345678901234x0123456789012345", "0123456789012345"),
        ("", "01234567890123456789"),
        ("32145678890123456789", "01234567890123456789"),
        ("01234567890123456789", "01234567890123456789"),
        ("x01234567890123456789", "01234567890123456789"),
        ("x0123456789012345678x01234567890123456789", "01234567890123456789"),
        ("xyz01234567890123456789", "01234567890123456789"),
        ("", "0123456789012345678901234567890"),
        ("321456788901234567890123456789012345678911", "0123456789012345678901234567890"),
        ("0123456789012345678901234567890", "0123456789012345678901234567890"),
        ("x0123456789012345678901234567890", "0123456789012345678901234567890"),
        ("x012345678901234567890123456789x0123456789012345678901234567890", "0123456789012345678901234567890"),
        ("xyz0123456789012345678901234567890", "0123456789012345678901234567890"),
        ("", "01234567890123456789012345678901"),
        ("32145678890123456789012345678901234567890211", "01234567890123456789012345678901"),
        ("01234567890123456789012345678901", "01234567890123456789012345678901"),
        ("x01234567890123456789012345678901", "01234567890123456789012345678901"),
        ("x0123456789012345678901234567890x01234567890123456789012345678901", "01234567890123456789012345678901"),
        ("xyz01234567890123456789012345678901", "01234567890123456789012345678901"),
        ("xxxxxx012345678901234567890123456789012345678901234567890123456789012", "012345678901234567890123456789012"),
        ("", "0123456789012345678901234567890123456789"),
        ("xx012345678901234567890123456789012345678901234567890123456789012", "0123456789012345678901234567890123456789"),
        ("xx012345678901234567890123456789012345678901234567890123456789012", "0123456789012345678901234567890123456789"),
        ("xx012345678901234567890123456789012345678901234567890123456789012", "0123456789012345678901234567890123456xxx"),
        ("xx0123456789012345678901234567890123456789012345678901234567890120123456789012345678901234567890123456xxx", "0123456789012345678901234567890123456xxx"),
        ("oxoxoxoxoxoxoxoxoxoxoxoy", "oy"),
        ("oxoxoxoxoxoxoxoxoxoxoxox", "oy"),

    ] {
        let text = text.to_string();
        let pattern = pattern.to_string();

        let kmp_matches = kmp::kmp(text.clone(), pattern.clone());
        let bm_matches: Vec<usize> = boyer_moore::boyer_moore(text.clone(), pattern.clone());
        let rk_matches: Vec<usize> = rabin_karp::rabin_karp(text.clone(), pattern.clone());

        assert_eq!(kmp_matches, bm_matches);
        assert_eq!(kmp_matches, rk_matches);
    }

    println!("String matching tests passed");
}
