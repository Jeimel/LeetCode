impl Solution {
    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        let mut sum = 0;

        'outer: for k in 1..=nums.len() {
            if nums.len() % k != 0 {
                continue;
            }

            let mut previous_max = i32::MIN;

            for chunk in nums.chunks(k) {
                let (mut breaks, mut min, mut max) = (0, i32::MAX, i32::MIN);

                for i in 0..chunk.len() {
                    min = min.min(chunk[i]);
                    max = max.max(chunk[i]);
                    breaks += usize::from(chunk[i] > chunk[(i + 1) % chunk.len()]);
                }

                if breaks > 1 || previous_max > min {
                    continue 'outer;
                }

                previous_max = max;
            }

            sum += k;
        }

        sum as i32
    }
}
