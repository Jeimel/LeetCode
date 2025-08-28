impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix = vec![1; nums.len()];

        // We calculate the product of all indices up to i
        for i in 1..nums.len() {
            prefix[i] = prefix[i - 1] * nums[i - 1];
        }

        // Suffix product starts at neutral element of multiplication
        let mut suffix = 1;

        // Iterate in reverse order to calculate the suffix product,
        // which equivalent to the product of all elements greater than i.
        // By multiplying the suffix with the prefix, we obtain the product
        // of all elements except the element at index i.
        for i in (0..nums.len()).rev() {
            // The result is calculated first, as the element at index i 
            // must not be included into the product
            prefix[i] *= suffix;
            suffix *= nums[i];
        }

        prefix
    }
}
