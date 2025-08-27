impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, mut k: i32) {
        // The rotation is cyclic with order len(nums),
        // as rotating by len(nums) results in the same array
        let k = k as usize % nums.len();

        // Rotating an array results in the last k elements being
        // at the front. So reversing the whole array places these
        // elements in the front, but in reversed order
        nums.reverse();
        // Fix order of the original last k elements
        nums[..k].reverse();
        // Fix oder of the original first len(nums) - k elements,
        // which do not wrap around in the rotation
        nums[k..].reverse();
    }
}
