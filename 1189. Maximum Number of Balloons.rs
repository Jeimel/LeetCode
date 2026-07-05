impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let (mut frequency, mut count) = ([0; 26], 0);

        for letter in text.bytes() {
            frequency[usize::from(letter - b'a')] += 1;
        }

        loop {
            for letter in "balloon".bytes() {
                let i = usize::from(letter - b'a');

                if frequency[i] == 0 {
                    return count;
                }

                frequency[i] -= 1;
            }

            count += 1;
        }
    }
}
