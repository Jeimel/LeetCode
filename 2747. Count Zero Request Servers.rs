impl Solution {
    pub fn count_servers(n: i32, mut logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
        logs.sort_unstable_by_key(|log| log[1]);

        let mut sorted: Vec<(usize, &i32)> = queries.iter().enumerate().collect();
        sorted.sort_unstable_by_key(|query| query.1);

        let mut result = vec![0; sorted.len()];
        let (mut count, mut distinct) = (vec![0; n as usize], 0);
        let (mut i, mut j) = (0, 0);

        for (k, &query) in sorted {
            let lower = query - x;

            while j < logs.len() && logs[j][1] <= query {
                let server = logs[j][0] as usize - 1;
                count[server] += 1;
                distinct += i32::from(count[server] == 1);

                j += 1;
            }

            while i < logs.len() && logs[i][1] < lower {
                let server = logs[i][0] as usize - 1;
                count[server] -= 1;
                distinct -= i32::from(count[server] == 0);

                i += 1;
            }

            result[k] = n - distinct;
        }

        result
    }
}
