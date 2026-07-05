impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let (mut sum, k) = (0, k as usize);

        for i in 0..tickets.len() {
            sum += tickets[i].min(tickets[k] - i32::from(i > k));
        } 

        sum
    }
}
