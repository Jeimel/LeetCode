impl Solution {
    pub fn minimum_cost(cost1: i32, cost2: i32, cost_both: i32, need1: i32, need2: i32) -> i64 {
        if cost1 + cost2 < cost_both {
            return need1 as i64 * cost1 as i64 + need2 as i64 * cost2 as i64;
        }

        let diff = (need1 - need2) as i64;

        if diff < 0 {
            return need1 as i64 * cost_both as i64 + diff.abs() * cost2.min(cost_both) as i64;
        }

        need2 as i64 * cost_both as i64 + diff * cost1.min(cost_both) as i64
    }
}
