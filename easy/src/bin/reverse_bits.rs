fn main() {
    let bits: u32 = 0b00000010100101000001111010011100;

    println!("{}", reverse_bits(bits));
}

fn reverse_bits(x: u32) -> u32 {
    let mut x = x;

    // Get number as array of bytes
    let mut bytes_vec: Vec<u8> = vec![];

    for i in (0..32).rev() {
        let val = u32::pow(2, i);

        if x >= val {
            bytes_vec.push(1);
            x -= val;
        } else {
            bytes_vec.push(0);
        }
    }

    let mut result = 0;

    for i in 0..32 {
        if bytes_vec[i] == 1 {
            result += u32::pow(2, i as u32);
        }
    }

    result
}