fn main() {
    let s = String::from("1234111");

    println!("{}", num_decodings(s));
}

fn num_decodings(s: String) -> i32 {
    fn check(s: &Vec<char>, idx: usize) -> bool {
        // decided current element can combine with previous element or not
        match s[idx - 1] {
            '1' => true,
            '2' => match s[idx] {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' => true,
                _ => false
            },
            _ => false
        }
    }

    // dp solution
    let s = s.chars().into_iter().collect::<Vec<char>>();
    if s.len()==1 {
        return if s[0]!='0' {1} else {0}
    }

    // (# one char, # two char)
    let mut state = if s[0]=='0' {(0,0)} else {(1,0)};
    for i in 1..s.len() {
        state = if check(&s, i) {
            (if s[i]!='0' {state.0 + state.1} else {0}, state.0)
        } else {
            (if s[i]!='0' {state.0 + state.1} else {0}, 0)
        };
    }
    state.0 + state.1
}