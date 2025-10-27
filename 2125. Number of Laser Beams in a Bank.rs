impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let (mut result, mut previous) = (0, 0);

        for i in 0..bank.len() {
            let current = bank[i]
                .chars()    
                .filter(|&c| c == '1')
                .count();

            result += current * previous;

            if current != 0 {
                previous = current;
            }
        }

        result as i32
    }
}
