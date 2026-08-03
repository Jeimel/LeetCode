impl Solution {
    pub fn stone_game_iii(mut stone_value: Vec<i32>) -> String {
        stone_value.extend_from_slice(&[0, 0, 0]);

        let mut dp = vec![0; stone_value.len()];

        for i in (0..(stone_value.len() - 3)).rev() {
            dp[i] = (0..3)
                .scan(0, |sum, k| {
                    *sum += stone_value[i + k];
                    Some(*sum - dp[i + k + 1])
                })
                .max()
                .unwrap();
        }

        match dp[0] {
            score if score > 0 => "Alice",
            score if score < 0 => "Bob",
            _ => "Tie",
        }
        .to_string()
    }
}
