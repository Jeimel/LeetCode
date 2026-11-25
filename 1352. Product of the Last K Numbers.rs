struct ProductOfNumbers {
    prefix: Vec<i32>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        Self { prefix: vec![1] }
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            self.prefix.clear();
            self.prefix.push(1);
            return;
        }

        let previous = self.prefix.len() - 1;
        self.prefix.push(num * self.prefix[previous]);
    }

    fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;

        if self.prefix.len() <= k {
            return 0;
        }

        let r = self.prefix.len() - 1;

        self.prefix[r] / self.prefix[r - k]
    }
}
