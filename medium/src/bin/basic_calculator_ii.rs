fn main(){
    let s = "5/2*3 + 4*3/2 - 5*2/3 + 5/2*6 - 7/3*2".to_string();
    // 14/3/2

    println!("{}", calculate(s));
}

fn calculate(s: String) -> i32 {
    fn evaluate(exp: Vec<char>) -> i32 {
        if exp.contains(&'+') {
            let comps: Vec<&[char]> = exp.split(|&char| char == '+').collect();

            let mut result = 0;

            for comp in comps {
                result += evaluate(comp.to_vec());
            }

            return result;
        }

        if exp.contains(&'-') {
            let comps: Vec<&[char]> = exp.split(|&char| char == '-').collect();

            let mut result = evaluate(comps[0].to_vec());

            for i in 1..comps.len() {
                result -= evaluate(comps[i].to_vec());
            }

            return result;
        }

        let mut ptr = exp.len() - 1;

        while 0 < ptr {
            match exp[ptr] {
                '*' => {
                    let comps = exp.split_at(ptr);
                    return evaluate(comps.0.to_vec()) * evaluate(comps.1[1..].to_vec());
                },
                '/' => {
                    let comps = exp.split_at(ptr);
                    return evaluate(comps.0.to_vec()) / evaluate(comps.1[1..].to_vec());
                },
                _ => {ptr -= 1;}
            }
        }

        exp.iter().collect::<String>().parse::<i32>().unwrap()
    }

    let mut s: Vec<char> = s.chars().collect();

    s.retain(|&char| char != ' ');

    evaluate(s)
}
