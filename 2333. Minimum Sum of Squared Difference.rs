use std::collections::BTreeMap;

impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let (mut map, mut sum) = (BTreeMap::new(), 0);

        for i in 0..nums1.len() {
            let diff = (nums1[i] - nums2[i]).abs() as i64;

            sum += diff * diff;
            map.entry(diff).and_modify(|count| *count += 1).or_insert(1);
        }

        let mut k = (k1 + k2) as i64;

        while k > 0 {
            let (num, count) = map.pop_last().unwrap();
            if num == 0 {
                return 0;
            }

            let change = k.min(count);
            k -= change;

            *map.entry(num - 1).or_default() += change;

            sum -= (2 * num - 1) * change;
        }

        sum
    }
}
