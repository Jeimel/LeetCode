impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let mut dp = vec![false; n as usize + 1];

        for i in 1..dp.len() {
            let mut j = 1;

            while j * j <= i {
                if !dp[i - j * j] {
                    dp[i] = true;
                    break;
                }
                
                j += 1;
            } 
        }

        *dp.last().unwrap()
    }
}
