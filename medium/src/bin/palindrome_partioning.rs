fn main(){
    let s = String::from("aaaaaaaaaaaaaaaa");

    println!("{:?}", partition(s));
}

fn partition(s: String) -> Vec<Vec<String>> {
    fn mine(remains: Vec<&str>, temp: Vec<String>, result: &mut Vec<Vec<String>>) {
        if remains.len() == 0 {
            (*result).push(temp);
            return;
        }

        for reach in 1..remains.len() + 1 {
            let slice = remains[0..reach].to_vec();
            let mut slice_rev = slice.to_owned();
            slice_rev.reverse();

            if slice == slice_rev {
                let mut new_temp = Vec::from(temp.to_owned());
                new_temp.push(slice.join(""));
                mine(remains[reach..].to_vec(), new_temp, result);
            }
        }
    }
    let mut vec: Vec<&str> = s.split("").collect();
    vec.retain(|&str| str != "");

    let mut result: Vec<Vec<String>> = vec![];

    mine(vec, vec![], &mut result);

    result
}

// Take a Vec<&str>
// Take a slice from it
// Check if the slice is a palindrome
// If it is, add it to a temporary array
// Pass the temporary array AND remaining Vec<&str> to recursive function
// When end is reached, push the temporary array to the final "result" array