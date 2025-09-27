impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut max_area = 0f64;

        // Just brute force every possible combination, but 
        // we can skip already tried possibilities 
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                for k in (j + 1)..points.len() {
                    let (x1, y1) = (points[i][0], points[i][1]);
                    let (x2, y2) = (points[j][0], points[j][1]);
                    let (x3, y3) = (points[k][0], points[k][1]);

                    // We construct two vectors 
                    // AB = B - A = (x2 - x1, y2 - y1)
                    // AC = C - A = (x3 - x1, y3 - y1)

                    // Now, the magnitude of their cross product gives
                    // the area of the parallelogram formed by them, 
                    // which we dived by two as we need the triangle
                    let area = 0.5 * ((x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1)) as f64;

                    // Did we find a new maximum triangle?
                    max_area = max_area.max(area.abs());
                }
            }
        }

        max_area
    }
}
