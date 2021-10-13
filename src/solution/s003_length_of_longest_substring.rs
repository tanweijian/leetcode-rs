use crate::solution::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let (mut start, mut end, mut max) = (0, 0, 0);

        while end < chars.len() {
            for idx in start..end {
                if chars[idx] == chars[end] {
                    start = idx + 1;
                    break;
                }
            }
            let len = end - start + 1;
            if len > max {
                max = len
            }
            end += 1
        }
        max as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;

    #[test]
    fn s003() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring(String::from("abcabcbb"))
        );
        assert_eq!(
            1,
            Solution::length_of_longest_substring(String::from("bbbbb"))
        );
        assert_eq!(
            3,
            Solution::length_of_longest_substring(String::from("pwwkew"))
        );
        assert_eq!(0, Solution::length_of_longest_substring(String::from("")));
    }
}
