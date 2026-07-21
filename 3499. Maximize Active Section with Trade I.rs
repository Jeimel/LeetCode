impl Solution {
    pub fn max_active_sections_after_trade(s: String) -> i32 {
        let (mut current, mut previous, mut ones, mut last, mut max) = (0, 0, 0, true, 0);

        for &b in s.as_bytes() {
            if b == b'1' {
                ones += 1;

                if !last {
                    previous = current;
                    current = 0;
                    last = true;
                }
            } else {
                current += 1;
                last = false;
            }

            if current > 0 && previous > 0 {
                max = max.max(previous + current);
            }
        }

        max + ones
    }
}
