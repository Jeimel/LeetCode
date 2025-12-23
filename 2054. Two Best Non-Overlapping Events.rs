impl Solution {
    pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort_unstable_by_key(|event| event[0]);

        let mut suffix = vec![0; events.len()];
        suffix[events.len() - 1] = events[events.len() - 1][2];

        // We calculate largest value after the current index.
        // Because `events` is sorted by start-time, we know that
        // every event after the current index is not overlapping
        // with an end-time of a previous index
        for i in (0..(events.len() - 1)).rev() {
            suffix[i] = suffix[i + 1].max(events[i][2]);
        }

        let mut max = 0;

        for i in 0..events.len() {
            // Find the index of the first event, where the end- and
            // start-time are not overlapping with the current index, 
            // which we can then use to index `suffix´ to get the 
            // largest non-overlapping value
            let partition = i + 1 + events[(i + 1)..]
                .partition_point(|event| event[0] <= events[i][1]);

            if partition < events.len() {
                max = max.max(events[i][2] + suffix[partition]);
            }

            // If all elements are overlapping, we have to return
            // the largest value from `events`
            max = max.max(events[i][2]);
        }

        max
   }
}
