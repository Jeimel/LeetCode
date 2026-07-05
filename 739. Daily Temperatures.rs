impl Solution {
    pub fn daily_temperatures(mut temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();

        for i in 0..temperatures.len() {
            while let Some(&top) = stack.last() && temperatures[top] < temperatures[i] {
                let top = stack.pop().unwrap();
                temperatures[top] = (i - top) as i32;
            }
           
            stack.push(i);
        }

        for i in stack {
            temperatures[i] = 0;
        }

        temperatures
    }
}
