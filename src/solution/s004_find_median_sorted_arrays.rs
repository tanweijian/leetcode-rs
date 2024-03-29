use crate::solution::Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let length = nums1.len() + nums2.len();
        if length & 1 == 1 {
            let median = 0_f64;
            median
        } else {
            let median1 = 0_f64;
            let median2 = 0_f64;
            (median1 + median2) / 2_f64
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;

    #[test]
    fn s004() {
        assert_eq!(
            2_f64,
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2])
        );
        assert_eq!(
            2.5_f64,
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4])
        );
    }
}
