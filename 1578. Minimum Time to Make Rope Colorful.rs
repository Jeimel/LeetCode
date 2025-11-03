impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let (bytes, mut time, mut i) = (colors.as_bytes(), 0, 1);

        while i < bytes.len() {
            // Our goal is to remove all balloons with the same color, except
            // the balloon with the highest time necessary to remove. So we 
            // track the sum and maximum of time for all consecutive balloons
            // with the same color
            let (mut max, mut sum) = (needed_time[i - 1], needed_time[i - 1]);

            while i < bytes.len() && bytes[i] == bytes[i - 1] {
                max = max.max(needed_time[i]);
                sum += needed_time[i];

                i += 1;
            }

            // We remove all balloons except the balloon with the highest time
            time += sum - max;
            i += 1;
        }
        
        time 
    }
}1578. Minimum Time to Make Rope Colorful
