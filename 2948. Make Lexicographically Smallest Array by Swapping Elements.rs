impl Solution { 
    pub fn lexicographically_smallest_array(mut nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let mut sorted: Vec<(i32, usize)> = nums.iter().enumerate().map(|(i, num)| (*num, i)).collect();
        sorted.sort_unstable();

        let (mut values, mut indices) = (Vec::new(), Vec::new());
        values.push(sorted[0].0);
        indices.push(sorted[0].1);
        
        for i in 1..nums.len() {
            if sorted[i].0 - sorted[i - 1].0 > limit {
                Self::swap(&mut nums, &mut values, &mut indices);

                values.clear();
                indices.clear();
            }

            values.push(sorted[i].0);
            indices.push(sorted[i].1);
        }

        Self::swap(&mut nums, &mut values, &mut indices);

        nums
    }

    fn swap(nums: &mut Vec<i32>, values: &mut Vec<i32>, indices: &mut Vec<usize>) {
        values.sort_unstable();
        indices.sort_unstable();

        for i in 0..values.len() {
            nums[indices[i]] = values[i];
        }
    }
}
