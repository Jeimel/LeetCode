impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        const SIZE: usize = 26;

        let mut frequencies = [0i32; SIZE];

        for b in s.as_bytes() {
            frequencies[(b - b'a') as usize] += 1;
        }

        let (mut max_vowel, mut max_consonant) = (i32::MIN, i32::MIN);

        for i in 0..SIZE {
            if matches!(i as u8 + b'a', b'a' | b'e' | b'i' | b'o' | b'u') {
                max_vowel = max_vowel.max(frequencies[i]);

                continue;
            }

            max_consonant = max_consonant.max(frequencies[i]);
        }

        max_vowel + max_consonant
    }
}
