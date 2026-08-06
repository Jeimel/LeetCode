impl Solution {
    pub fn smallest_number(mut n: i32, t: i32) -> i32 {
        loop {
            let (mut product, mut m) = (1, n);
            
            while m != 0 {
                product *= m % 10;
                m /= 10;
            }

            if product % t == 0 {
                return n;
            }

            n += 1;
        }
    }
}
