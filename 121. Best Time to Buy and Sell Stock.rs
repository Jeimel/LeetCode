impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // We have to find maximum difference between consecutive elements, 
        // so we have to keep track of the smallest element and the highest 
        // difference to it.
        let (mut buy, mut profit) = (prices[0], 0);

        for i in 1..prices.len() {
            // Have we found the highest profit so far?
            profit = profit.max(prices[i] - buy);
            // Have we a new minimum element?
            buy = buy.min(prices[i]);
        }

        profit
    }
}
