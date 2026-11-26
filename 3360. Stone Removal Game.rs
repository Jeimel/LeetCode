impl Solution {
    pub fn can_alice_win(mut n: i32) -> bool {
        let mut turn = false;

        for i in (0..11).rev() {
            n -= i;
            if n < 0 {
                return turn;
            }

            turn = !turn;
        }

        turn
    }
}
