use std::cmp::Ordering;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // i and j iterate over their respecteive arrays in reverse order
        let (mut i, mut j) = (m as usize - 1, n as usize - 1);

        // Iterate over (m + n) in inverse order while inserting 
        // the biggest element left from both arrays at each step
        for k in (0..(m + n) as usize).rev() {
            // Determine the biggest element from both arrays based on i and j, 
            // and decrease the given pointer
            match nums1.get(i).cmp(&nums2.get(j)) { 
                Ordering::Less => {
                    nums1[k] = nums2[j];
                    j -= 1;
                },
                _ => {
                    nums1[k] = nums1[i];
                    i -= 1;
                }
            }
        }
    }
}
