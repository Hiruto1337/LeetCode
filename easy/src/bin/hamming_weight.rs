fn main(){
    let bits: u32 = 0b00000000000000000000000000001011;

    println!("{}", hammingWeight(bits));
}

fn hammingWeight (n: u32) -> i32 {
    let mut n = n;

    let mut result = 0;

    for i in (0..32).rev() {
        let val = u32::pow(2, i);

        if n >= val {
            result += 1;
            n -= val;
        }
    }

    result
}