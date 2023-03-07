fn main() {
    let s = String::from("1210");

    let result = num_decodings(s);

    println!("{result}");
}

fn num_decodings(s: String) -> i32 {
    fn can_combine(list: &Vec<char>, index: usize) -> bool {
        match list[index - 1] {
            '1' => true,
            '2' => match list[index] {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' => true,
                _ => false
            },
            _ => false,
        }
    }

    let s: Vec<char> = s.chars().collect();
    if s[0] == '0' {
        return 0;
    }

    let mut comb_single_digit = 1;
    let mut comb_double_digit = 0;

    for i in 1..s.len() {
        if can_combine(&s, i) {
            if s[i] != '0' {
                // s[i] can serve both as a single digit and as part of a double digit
                // Thus, we consider both posibilities
                let all_comb = comb_single_digit + comb_double_digit;
                comb_double_digit = comb_single_digit;
                comb_single_digit = all_comb;
            } else {
                // s[i] can ONLY serve as part of a double digit
                comb_double_digit = comb_single_digit;
                comb_single_digit = 0;
            }
        } else {
            if s[i] != '0' {
                // s[i] can ONLY serve as a single digit
                comb_single_digit = comb_single_digit + comb_double_digit;
                comb_double_digit = 0;
            } else {
                // Impossible case
                return 0;
            }
        }
    }

    comb_single_digit + comb_double_digit
}

// 1
// A

// 11
// AA, K

// 111
// AAA, KA, AK

// 1111
// AAAA, KAA, AKA, AAK, KK

// 11110
// AAAJ, AKJ, KAJ

// 11111
// AAAAA, KAAA, AKAA, AAKA, AAAK, KAAK, AAKK, KKK

// 111110
// AAAAJ, KAAJ, AKAJ, AAKJ, KKJ

// 11131
// AAACA, KACA, AKCA, AAMA, KMA

// 11121
// AAABA, KABA, AKBA, AAAU, AALA, KAU, AKU, KLA

// 1234111
// ABCDAAA, ABCDKA, ABCDAK, LCDAAA, LCDKA, LCDAK, AWDAAA, AWDKA, AWDAK
