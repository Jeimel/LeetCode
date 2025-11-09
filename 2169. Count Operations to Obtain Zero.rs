impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        if num1 * num2 == 0 {
            return 0;
        }

        num1 / num2 + Self::count_operations(num2, num1 % num2)
    }
}
