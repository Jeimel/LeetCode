impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let (mut sum, mut contiguous) = (1, 1);

        for i in 1..prices.len() {
            contiguous = if prices[i - 1] - prices[i] == 1 {
                contiguous + 1
            } else {
                1
            };

            sum += contiguous;
        }

        sum
    }
}
