impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let (mut zeros, mut ones) = (0, 0);

        for i in 0..students.len() {
            zeros += students[i];
            ones += 1 - students[i];
        }

        for i in 0..sandwiches.len() {
            zeros -= sandwiches[i];
            ones -= 1 - sandwiches[i];

            if zeros < 0 || ones < 0 {
                return (students.len() - i) as i32;
            }
        }

        0
    }
}
