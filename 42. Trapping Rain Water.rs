impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut dp, mut sum) = (vec![0; height.len()], 0);

        for i in 1..(height.len() - 1) {
            // Each dp state stores the amount of water traped at bar `i`.
            // Therefore, the height of the left bar is its height plus the 
            // stored water, which we can compare against our current bar
            dp[i] = (height[i - 1] + dp[i - 1] - height[i]).max(0);
            sum += dp[i];
        }

        let mut max = height[height.len() - 1];

        // In the first iteration, we only compare against the left neighbor.
        // So, we need to remove every element of water, which isn't trapped
        // due to a smaller right neighbor. This is only necessary until we
        // found the first bar not trapping any water, as this blocks every 
        // water source to its left.
        for i in (1..(height.len() - 1)).rev() {
            if dp[i] == 0 {
                break;
            }

            max = max.max(height[i]);
            // We can at most store `max` water, as `max` represents the
            // highest bar starting from the rightmost index
            sum -= (dp[i] + height[i] - max).max(0);
        }

        sum
    }
}
