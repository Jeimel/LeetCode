use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort_unstable_by_key(|meeting| meeting[0]);

        let n = n as usize;

        let mut free = BinaryHeap::new();
        let mut occupied = BinaryHeap::new();
        let mut usage = vec![0; n];
        let mut time = 0;

        for i in 0..n {
            free.push(Reverse(i));
        }

        for meeting in meetings {
            time = time.max(meeting[0] as i64);

            // We remove all elements smaller than the current time, or
            // at least one element with a respective time update
            while let Some(&Reverse((end, room))) = occupied
                .peek()
                .filter(|&&Reverse((end, _))| free.is_empty() || end <= time)
            {
                free.push(Reverse(room));
                occupied.pop();
                time = time.max(end);
            }

            // Use the smallest available room for the meeting
            let Reverse(room) = free.pop().unwrap();
            usage[room] += 1;

            // Mark the `room` as occupied until the meeting ends
            occupied.push(Reverse((time - meeting[0] as i64 + meeting[1] as i64, room)));
        }

        (0..n).max_by_key(|&i| (usage[i], Reverse(i))).unwrap() as i32
    }
}
