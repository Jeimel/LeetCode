impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        // We find the maximum among all kids first
        let max = *candies.iter().max().unwrap();

        // Then, we check if each kid reaches `max` with `extra_candies` included
        candies
            .iter()
            .map(|kid| kid + extra_candies >= max)
            .collect()
   }
}
