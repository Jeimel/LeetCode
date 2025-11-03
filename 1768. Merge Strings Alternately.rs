use itertools::{Itertools, EitherOrBoth::{Both, Left, Right}};

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::with_capacity(word1.len() + word2.len());

        // We just iterate until both strings are empty, and in each step
        // we push the left first if possible
        for pair in word1.chars().zip_longest(word2.chars()) {
            match pair {
                Both(l, r) => { result.push(l); result.push(r); },
                Left(l) => result.push(l),
                Right(r) => result.push(r),
            };
        }

        result
    }
}
