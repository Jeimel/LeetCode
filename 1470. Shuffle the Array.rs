mpl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mid = nums.len() / 2;

        (0..mid).fold(Vec::new(), |mut acc, i| {
            acc.push(nums[i]);
            acc.push(nums[i + mid]);
            acc
        })
    }
}
