impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let (mut result, mut i) = (vec![], 0);

        for j in 1..=n {
            if i >= target.len() {
                break;
            }

            result.push("Push".to_string());

            if j == target[i] {
                i += 1;
            } else {
                result.push("Pop".to_string());
            }
        }

        result
    }
}
