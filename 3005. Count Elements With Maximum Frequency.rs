use std::cmp::Ordering;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        // We store all frequencies, the currently highest frequency, 
        // and the amount of occurences for the highest frequency
        let (mut frequencies, mut max, mut count) = ([0; 100], 0, 0);

        for num in nums {
            let i = num as usize - 1;

            // We found another occurrence, so we increase the counter
            frequencies[i] += 1;

            match frequencies[i].cmp(&max) {
                // We found a new maximum frequency, so we update the maximum, and set
                // the count equal to the amount of occurences of the new maximum
                Ordering::Greater => { max = frequencies[i]; count = frequencies[i]; }
                // If two maximum frequencies are equal, we know that the numbers they are
                // representing are unequal, so we have to add all occurences of this 
                // new number to the total amount
                Ordering::Equal => count += frequencies[i],
                // Frequency is lower than our current maximum, so we have to do nothing
                _ => {},
            }
        }

        count 
    }
}
