use std::cmp::Ordering;

impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let (mut max_diagonal, mut max_area) = (0, 0);

        for rectangle in dimensions {
            // We don't have to take the square root for comparison,
            // due to the fact that diagonal will always be greater than 1
            let diagonal = rectangle[0] * rectangle[0] + rectangle[1] * rectangle[1];

            match max_diagonal.cmp(&diagonal) {
                // New maximum diagonal
                Ordering::Less => { 
                    max_diagonal = diagonal; 
                    max_area = rectangle[0] * rectangle[1]; 
                },
                // The area for two rectangles with equal diagonal can differ
                Ordering::Equal => max_area = max_area.max(rectangle[0] * rectangle[1]),
                Ordering::Greater => {},
            }
        }

        max_area
    }
}
