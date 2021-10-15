use leetcode_rs::solution::Solution;

fn main() {
    let s = String::from("abcabcbb");
    let output = Solution::length_of_longest_substring(s.clone());
    println!("Input: s = {}", s);
    println!("Output: {}", output);
    println!();

    let s = String::from("bbbbb");
    let output = Solution::length_of_longest_substring(s.clone());
    println!("Input: s = {}", s);
    println!("Output: {}", output);
    println!();

    let s = String::from("pwwkew");
    let output = Solution::length_of_longest_substring(s.clone());
    println!("Input: s = {}", s);
    println!("Output: {}", output);
}
