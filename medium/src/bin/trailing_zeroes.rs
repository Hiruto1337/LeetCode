fn main(){
    println!("{}", trailing_zeroes(10));
}

fn trailing_zeroes(n: i32) -> i32 {
    fn vecify(n: i32) -> Vec<u8> {
        let vec_chars: Vec<char> = n.to_string().chars().collect();

        let mut vec: Vec<u8> = vec![];

        for char in vec_chars {
            vec.push(char.to_digit(10).unwrap() as u8);
        }

        vec
    }
    fn multiply(num1: Vec<u8>, num2: Vec<u8>) -> Vec<u8> {
        println!("{num1:?}");
        println!("{num2:?}");
        vec![]
    }

    let mut total = vecify(n);

    let num1 = vecify(12);
    let num2 = vecify(12);

    multiply(num1, num2);

    // for i in (1..n - 1).rev() {
    //     total = multiply(total, vecify(i));
    // }

    -1
}

// Convert the number to a vector of chars, Vec<char>
// Create a function that can multiply two Vec<char> and returns the resulting Vec<char>
// Create a loop where num_total is multiplied with (n-1)_as_vec,
// This loop must run, until n == 0
// Search from right to left, until a num is not 0