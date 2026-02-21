/*

Practice Question

Write a program that reads the string 𝑠
 and prints the suffix array 𝑝
 for it.

Input
Input contains a single string 𝑠
 of length 𝑛
 (1≤𝑛≤100000
). String consists of small english letters.

Output
Print 𝑛+1
 distinct integers, indices of first characters of suffixes of 𝑠
, ordered in lexicographical order.

example 1:
input: ababba
output: 6 5 0 2 4 1 3


*/

pub fn test_suffix_array() {
    let inputs: Vec<&str> = vec!["ababba", "aaaa", "ppppplppp", "nn"];

    for n in 0..4 {
        suffix_array_solution(inputs[n]);
    }
}

pub fn suffix_array_solution(input: &str) {
    println!("test input: {}", input);

    /*
    0. ababba
    1. babba
    2. abba
    3. bba
    4. ba
    5. a
    6. ""

    arranged
    6 5 0 2 4 1 3

    pseudo code:
    * take the string and split them into a splice
    * iterate through the splice and assign indexs in ascending order for n+1
    * create a new splice with all suffixes included
    * sort through the splice in lexographical order

    "ChatGPT answer"
    * generate all n+1 suffixes
    * keep track of their starting indices
    * sort suffixes lexicographically
    * output the sorted indices

    */

    /* Self Attemp
    let n: usize = input.len();
    let mut indices: Vec<(&str, usize)> = Vec::new();
    let mut arranged_indices: Vec<usize> = Vec::new();

    for index in 0..=n {
        let trimmed_input: &str = &input[index..n];
        indices.push((trimmed_input, index));
    }

    indices.sort();

    for index in 0..=n {
        arranged_indices.push(indices[index].1);
    }

    let output = arranged_indices
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", output);
    */

    /*
    Deeper Research Attempt
    sort n+1 elements -  0(n)
    complexity of sorting - 0 (n log n) -> hybrid quicksort + heapsort
    one comparison cost - 0(n)
    total - 0(n² log n)
    */

    let n = input.len();
    let mut indices: Vec<usize> = (0..=n).collect();

    indices.sort_by(|&a, &b| input[a..].cmp(&input[b..]));

    for i in indices {
        print!("{} ", i);
    }
    println!();
}
