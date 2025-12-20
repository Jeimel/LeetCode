use std::collections::HashMap;

struct UnionFind(Vec<usize>);

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self((0..n).collect())
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.0[x] != x {
            let root = self.find(self.0[x]);
            self.0[x] = root;
        }

        self.0[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);

        let (x, y) = if x < y { (x, y) } else { (y, x) };
        self.0[y] = x;
    }

    pub fn part(&mut self, x: usize) {
        self.0[x] = x;
    }

    pub fn rank(&mut self, rank: usize) -> Vec<i32> {
        (0..self.0.len())
            .filter(|&i| self.find(i) == 0)
            .map(|i| i as i32)
            .collect()
    }
}

impl Solution {
    pub fn find_all_people(n: i32, mut meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        meetings.sort_unstable_by_key(|meeting| meeting[2]);

        let mut uf = UnionFind::new(n as usize);
        uf.union(0, first_person as usize);

        for chunk in meetings.chunk_by(|a, b| a[2] == b[2]) {
            for meeting in chunk {
                uf.union(meeting[0] as usize, meeting[1] as usize);
            }

            for meeting in chunk {
                if uf.find(meeting[0] as usize) == 0 {
                    continue;
                }

                uf.part(meeting[0] as usize);
                uf.part(meeting[1] as usize);
            }
        }

        uf.rank(0)
    }
}
