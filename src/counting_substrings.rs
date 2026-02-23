/*
Given a string 𝑡
 and 𝑛
 queries, each query is a string 𝑠𝑖
. For each request you need to count how many times the string 𝑠𝑖
 occurs as a substring in 𝑡
.

Input
The first line of input contains the string 𝑡
 (1≤|𝑡|≤300,000
).

The second line contains an integer 𝑛
, the number of queries (1≤𝑛≤300,000
). The following 𝑛
 lines contain one non-empty line 𝑠𝑖
 each. The sum of the lengths of all strings 𝑠𝑖
 does not exceed 300,000
.

All strings consist of lowercase English letters.

Output
For each request print one integer, the number of times the string 𝑠𝑖
 occurs as a substring in 𝑡.
*/

use aho_corasick::AhoCorasick;

pub fn test_counting_substrings() {
    let input = vec![
        ("ababba", vec!["ba", "baba", "abba"]),
        ("itmouniversity", vec!["it", "more", "university"]),
        ("aaa", vec!["a", "aa"]),
    ];

    for (t, n) in input {
        counting_substrings_solution(t, &n);
    }
}

pub fn counting_substrings_solution(t: &str, n: &[&str]) {
    let ac = AhoCorasick::new(n).unwrap();
    let mut matches: Vec<u32> = vec![0; n.len()];

    for mat in ac.find_overlapping_iter(t) {
        let pattern_index = mat.pattern().as_usize();
        matches[pattern_index] += 1; 
    }

    for i in 0..matches.len() {
        println!("{}",matches[i]);
    }
}
