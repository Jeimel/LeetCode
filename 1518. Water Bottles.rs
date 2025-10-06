impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        // We need num_exchange empty bottles to receive 1 new full bottle. 
        // Once we drink it, we gain back 1 empty bottle. In other words,
        // the net loss per bottle is num_exchange - 1. 
        //
        // We start with num_bottles, so we can only start 
        // exchanging after num_exchange bottles
        num_bottles + (num_bottles - 1) / (num_exchange - 1)
    }
}
