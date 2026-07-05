impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
        arr.sort_unstable();

        (0..arr.len()).fold(0, |current, i| arr[i].min(current + 1))
   }
}
