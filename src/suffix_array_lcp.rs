/*
Try to implement the algorithm of Kasai, Arimura, Arikawa, Lee and Park find the values of 𝑙𝑐𝑝
 for the suffix array.

Build a suffix array for a given string 𝑠
, for each two adjacent suffixes find the length of longest common prefix.

Input
First line holds a single string 𝑠
 of length 𝑛
 (1≤𝑛≤400000
). String consists of small english letters.

Output
In the first line print 𝑛+1
 distinct integers, indices of first characters of suffixes of 𝑠
, ordered in lexicographical order.

In the second line print 𝑛
 integers, lengths of longest common prefixes.

e.g. input ababba
e.g. output
6 5 0 2 4 1 3
0 1 2 0 2 1

0. ababba
1. babba
2. abba
3. bba
4. ba
5. a
6. ""

*/

pub fn test_suffix_array_lcp() {
    let input = vec!["ababba", "aaaa", "ppppplppp", "nn"];

    for s in input {
        suffix_array_lcp_solution(s)
    }
}

pub fn suffix_array_lcp_solution(input: &str) {
    let mut suffixes: Vec<usize> = (0..=input.len()).collect();

    suffixes.sort_by(|&a, &b| input[a..].cmp(&input[b..]));

    let mut lcp_array = Vec::new();

    for k in 0..input.len() {
        let i = suffixes[k];
        let j = suffixes[k + 1];

        let length = lcp(input, i, j);
        lcp_array.push(length);
    }

    // for i in suffixes {
    //     print!("{} ", i);
    // }
    // println!();
}

fn lcp(s: &str, i: usize, j: usize) -> usize {
    let bytes = s.as_bytes();
    let mut len = 0;

    while i + len < bytes.len() && j + len < bytes.len() && bytes[i + len] == bytes[j + len] {
        len += 1;
    }

    len
}
