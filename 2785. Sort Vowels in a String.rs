impl Solution {
    pub fn sort_vowels(s: String) -> String {
        // All vowels sorted by ascending ascii value
        const VOWELS: [u8; 10] = [b'A', b'E', b'I', b'O', b'U', b'a', b'e', b'i', b'o', b'u'];

        // We count all ascii characters starting at 'A' to reduce the amount of memory used.
        // Therefore, we need to offset everything by 'A'
        let mut count = [0; (b'z' - b'A') as usize + 1];

        for c in s.bytes() {
            count[(c - b'A') as usize] += 1;
        }

        // We keep track of the currently checked vowel, which indexes the vowels array,
        // and this in turn can index the count array
        let (mut result, mut vowel) = (String::with_capacity(s.len()), 0);
   
        for c in s.bytes() {
            // If we have no vowel, we just add the char to the result
            if !VOWELS.contains(&c) {
                result.push(c as char);

                continue;
            }

            // We have to find the next vowel, which has a positive count
            while count[(VOWELS[vowel] - b'A') as usize] == 0 {
                vowel += 1;
            }
            
            // Push the next lowest vowel to the result
            result.push(VOWELS[vowel] as char);

            // Decrease the count, as we have included the vowel once
            count[(VOWELS[vowel] - b'A') as usize] -= 1;
        }

        result
    }
}
