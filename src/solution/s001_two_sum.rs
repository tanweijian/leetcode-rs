use crate::solution::Solution;
use std::collections::BTreeMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut bmap: BTreeMap<i32, usize> = BTreeMap::new();
        let mut result: Vec<i32> = Vec::with_capacity(2);
        for (i, &v) in nums.iter().enumerate() {
            let key = &(target - v);
            if bmap.contains_key(key) {
                let j = *bmap.get(key).unwrap();
                result.insert(0, j as i32);
                result.insert(1, i as i32);
                break;
            }
            bmap.insert(v, i);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn s001() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}
