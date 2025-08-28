impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        // The value of h is limited by len(citations).
        // Therefore, we can count all citations up to len(citations)
        let mut count = vec![0; citations.len() + 1];

        for &citation in &citations {
            // Due to the limitation of h, citations greater than len(citations)
            // are grouped together.
            count[(citation as usize).min(citations.len())] += 1;
        }

        // Track the total number of citations
        let mut total = 0;

        // Iterate in reverse, as we are searching for the highest h-index
        for i in (0..count.len()).rev() {
            total += count[i];

            // Has the researcher published at least i papers 
            // that have benn cited total times?
            if total >= i {
                return i as i32;
            }
        }

        0
    }
}
