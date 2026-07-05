impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut exclusive_time = vec![0; n as usize];
        let mut previous_time = 0;

        for log in logs {
            let mut log = log.split(':');

            let id = log.next().unwrap().parse::<usize>().unwrap();
            let status = log.next().unwrap();
            let time = log.next().unwrap().parse::<i32>().unwrap();

            if status == "start" {
                if let Some(current) = stack.last() {
                    exclusive_time[*current] += time - previous_time;
                };

                previous_time = time;
                stack.push(id);
            } else {
                exclusive_time[id] += time - previous_time + 1;

                previous_time = time + 1; 
                stack.pop();
            }
        }

        exclusive_time
    }
}
