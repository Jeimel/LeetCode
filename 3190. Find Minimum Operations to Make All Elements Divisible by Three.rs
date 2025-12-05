impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        // We have to consider three values of num % 3:
        // - 0: num is already divisible by 3
        // - 1: num - 1 is divisible by 3
        // - 2: num + 1 is divisible by 3 
        nums.iter().filter(|num| *num % 3 != 0).count() as i32
    }
}
