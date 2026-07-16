impl Solution {
    pub fn gcd_sum(nums: Vec<i32>) -> i64 {
        let (mut prefix_gcd, mut mx) = (Vec::new(), nums[0] as i64);

        for num in nums {
            mx = mx.max(num as i64);
            prefix_gcd.push(Self::gcd(num as i64, mx));
        }

        prefix_gcd.sort_unstable();

        (0..(prefix_gcd.len() / 2)).fold(0, |sum, i| {
            sum + Self::gcd(prefix_gcd[i], prefix_gcd[prefix_gcd.len() - 1 - i])
        })
    }

    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            (a, b) = (b, a % b);
        }

        a
    }
}
