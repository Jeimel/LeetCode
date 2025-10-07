use std::collections::{BTreeSet, HashMap};

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        // We use 1 as our default value, as we can always drain the first lake
        let (mut result, mut full, mut dry) = (vec![1; rains.len()], HashMap::new(), BTreeSet::new());

        for (today, lake) in rains.into_iter().enumerate() {
            // We store all days, where we can dry a lake and use those if necessary
            if lake == 0 {
                dry.insert(today);
                continue;
            }

            // We have to fill the lake, as rains[i] > 0
            result[today] = -1;

            let filled = full.insert(lake, today);

            // If the lake is not full of water, we can just fill it
            if filled.is_none() {
                continue;
            }

            match dry.range(filled.unwrap()..).next() {
                // We dry the lake using the smallest day, where we can dry 
                // and it comes after the last rain on the lake
                Some(&day) => {
                    result[day] = lake;
                    dry.remove(&day);
                },
                // If we don't have fitting spare day to dry anymore, the lake will flood
                None => return Vec::new(),
            }
       }

        result
    }
}
