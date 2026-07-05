impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        fn apply(a: i32, b: i32, op: char) -> i32 {
            match op {
                '+' => a + b,
                '-' => a - b,
                '*' => a * b,
                '/' => a / b,
                _ => unreachable!(),
            }
        };

        let mut stack = Vec::new();

        for token in tokens {
            match token.parse::<i32>() {
                Ok(digit) => stack.push(digit),
                Err(_) => {
                    let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
                    stack.push(apply(b, a, token.chars().next().unwrap()));
                },
            };
        }

        stack[0]
    }
}
