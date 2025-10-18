impl Solution {
    pub fn max_distinct_elements(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();

        let (mut distinct, mut previous) = (0, -1_000_000_000);

        // We iterate the sorted vector, and pick the smallest available number,
        // which is in range and greater than the previous element
        for num in nums {
            // We select the smallest value greater than `previous` inside our range
            let current = (num - k).max(previous + 1);

            // We can't build any new number fron `num`, as we exceed the range
            if current > num + k {
                continue;
            }

            // We found a valid number in range, and it's unique
            previous = current;
            distinct += 1;
        }

        distinct
    }
}
