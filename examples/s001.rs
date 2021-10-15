use leetcode_rs::solution::Solution;

fn main() {
    let input = vec![2, 7, 11, 15];
    let target = 9_i32;
    let output = Solution::two_sum(input.clone(), target);
    println!("Input: nums = {:?}, target = {}", input, target);
    println!("Output: {:?}", output);
    println!();

    let input = vec![3, 2, 4];
    let target = 6_i32;
    let output = Solution::two_sum(input.clone(), target);
    println!("Input: nums = {:?}, target = {}", input, target);
    println!("Output: {:?}", output);
    println!();

    let input = vec![3, 3];
    let target = 6_i32;
    let output = Solution::two_sum(input.clone(), target);
    println!("Input: nums = {:?}, target = {}", input, target);
    println!("Output: {:?}", output);
}
