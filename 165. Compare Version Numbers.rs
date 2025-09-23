use std::cmp::Ordering;
use itertools::{EitherOrBoth, Itertools};

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        // Create two iterators over every revision, and parse revision to an integer
        let version1 = version1.split('.').map(|s| s.parse::<i32>().unwrap());
        let version2 = version2.split('.').map(|s| s.parse::<i32>().unwrap());

        // We compare the two revisions, and check if any revision is higher.
        // If two revisions are equal, we continue until we either iterated every revision
        // or find a higher revision.
        for pair in version1.zip_longest(version2) {
            let (revision1, revision2) = match pair {
                EitherOrBoth::Both(revision1, revision2) => (revision1, revision2),
                EitherOrBoth::Left(revision1) => (revision1, 0),
                EitherOrBoth::Right(revision2) => (0, revision2),
            };

            match revision1.cmp(&revision2) {
                Ordering::Greater => return 1,
                Ordering::Equal => continue,
                Ordering::Less => return -1,
            };
        }

        0
    }
}
