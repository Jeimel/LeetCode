impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        // We just check if the string contains a '+' to determine increment or decrement
        operations.iter()
            .fold(0, |acc, operation| {
                acc + if operation.contains('+') { 1 } else { - 1 }
            }) 
    }
}
