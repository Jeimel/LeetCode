impl Solution {
    pub fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
        for i in 0..prices.len() - 1 {
            prices[i] = prices[i]
                - prices[(i + 1)..]
                    .iter()
                    .find(|&&x| x <= prices[i])
                    .unwrap_or(&0);
        }

        prices
    }
}
