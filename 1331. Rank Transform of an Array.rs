impl Solution {
    pub fn array_rank_transform(mut arr: Vec<i32>) -> Vec<i32> {
        let mut rank: Vec<(usize, i32)> = arr.iter().map(|num| *num).enumerate().collect();
        rank.sort_unstable_by_key(|(i, num)| *num);

        let mut current = 1;

        for i in 0..rank.len() {
            arr[rank[i].0] = current;

            if i != rank.len() - 1 && rank[i].1 != rank[i + 1].1 {
                current += 1;
            }
        }

        arr
    }
}
