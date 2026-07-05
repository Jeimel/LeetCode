impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack = Vec::new();

        let mut max = 0;

        for i in 0..heights.len() {
            while let Some(&top) = stack.last() && heights[top] > heights[i] {
                max = max.max(heights[stack.pop().unwrap()] * match stack.last() {
                    Some(&l) => i - l - 1,
                    None => i
                } as i32);
            }
           
            stack.push(i);
        }

        while let Some(top) = stack.pop() {
            max = max.max(heights[top] * (heights.len() - match stack.last() {
                Some(&l) => l + 1,
                None => 0
            }) as i32);
        }

        max
    }
}
