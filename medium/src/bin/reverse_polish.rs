fn main() {}

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut procedual_tokens: Vec<i32> = vec![];

    for token in tokens.into_iter() {
        if let Ok(num) = token.parse() {
            procedual_tokens.push(num);
        } else {
            let right = procedual_tokens.pop().unwrap();
            let left = procedual_tokens.pop().unwrap();

            match token.as_str() {
                "+" => {procedual_tokens.push(left + right)},
                "-" => {procedual_tokens.push(left - right)},
                "*" => {procedual_tokens.push(left * right)},
                "/" => {procedual_tokens.push(left / right)},
                _ => {}
            }
        }
    }

    procedual_tokens[0]
}
