impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let mut target = vec![(0, 0, 0); nums.len()];

        for i in 0..nums.len() {
            target[i].1 = nums[i];
            target[i].2 = i;

            let mut num = nums[i];

            while num != 0 {
                target[i].0 += num % 10;
                num /= 10;
            }
        }

        target.sort_unstable();

        let (mut swaps, mut visited) = (0, vec![false; nums.len()]);

        for i in 0..nums.len() {
            let (mut j, mut cycle) = (i, 0);

            while !visited[j] {
                visited[j] = true;

                j = target[j].2;
                cycle += 1;
            }

            swaps += 0.max(cycle - 1);
        } 

        swaps
    }
}
