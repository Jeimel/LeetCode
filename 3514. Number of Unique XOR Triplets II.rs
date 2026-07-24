impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let n = 1 << (32 - nums.iter().max().copied().unwrap_or(0).leading_zeros());

        let (mut exists, mut single) = (vec![false; n], Vec::new());

        for num in nums {
            let num = num as usize;
            if exists[num] {
                continue;
            }

            single.push(num);
            exists[num] = true;
        }

        let mut double = Vec::new();
        exists.fill(false);  

        for i in 0..single.len() {
            for j in i..single.len() {
                let xor = single[i] ^ single[j];
                if exists[xor] {
                    continue;
                }

                double.push(xor);
                exists[xor] = true;
            }
        }

        let mut count = 0;
        exists.fill(false);  

        for i in 0..single.len() {
            for j in 0..double.len() {
                let xor = single[i] ^ double[j];
                if exists[xor] {
                    continue;
                }

                count += 1;
                exists[xor] = true;
            }
        }

        count
    }
}
