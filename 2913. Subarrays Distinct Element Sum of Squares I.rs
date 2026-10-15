impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let (n, mut result) = (nums.len(), 0);

        for i in 0..n {
            let (mut set, mut unique) = (vec![false; 101], 0);

            for j in i..n {
                let num = nums[j] as usize;

                if !set[num] {
                    set[num] = true;
                    unique += 1;
                }

                result += unique * unique;
            }
        }

        result 
    }
}
