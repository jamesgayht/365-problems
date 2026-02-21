/*
Problem statement

Given a string 𝑡 and 𝑛 queries, each query is a string 𝑠𝑖. For each request you need to determine whether the string 𝑠𝑖 occurs as a substring in 𝑡.

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
For each request print "Yes" if the string 𝑠𝑖
 occurs in 𝑡
, and "No" otherwise.
*/

use aho_corasick::AhoCorasick;

pub fn test_substring_search() {
    let input: Vec<(&str, Vec<&str>)> = vec![
        ("ababba", vec!["ba", "baba", "abba"]),
        ("codeforces", vec!["code", "forces", "math"]),
        ("a", vec!["a", "a"]),
    ];

    for (t, n) in input {
        substring_search_solution(t, &n);
    }
}

pub fn substring_search_solution(t: &str, n: &[&str]) {
    /*
    for loop through n queries
    use contains() to n queries check against t
    print yes / no
    */

    /* Self Attempt - 0(n²) -> too slow

    for query in n {
        if t.contains(query) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
    */

    /*
    Deeper Research Attempt
    - initial thought was to use suffix array for binary search
    - later found out about aho corasick algo for multiple pattern matching
    - find_iter function ignores overlapping matches, got stuck for 30 mins, and finally found find_overlapping_iter() zz
    - final time complexity 0(n)
        O(300k + 300k) -> since both t and n are 300k 
        = O(600k)
        = O(n)
    */
    let ac = AhoCorasick::new(n).unwrap();

    let mut results = vec![false; n.len()];

    for mat in ac.find_overlapping_iter(t) {
        results[mat.pattern()] = true;
    }

    for result in results {
        if result {
            println!("yes");
        } else {
            println!("no");
        }
    }
}
