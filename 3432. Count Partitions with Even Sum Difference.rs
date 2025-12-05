impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        // We assume that x + y = sum, where x and y are the two subarrays.
        // If sum is odd, then either x or y is even, while the other is odd,
        // If sum is even, then either both are odd or both are even.
        if nums.iter().sum::<i32>() % 2 == 0 { nums.len() as i32 - 1 } else { 0 }
    }
}
