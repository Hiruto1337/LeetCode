fn main() {
    let mut nums1: Vec<i32> = vec![1];
    let m: i32 = 1;
    let mut nums2: Vec<i32> = vec![];
    let n: i32 = 0;

    merge(&mut nums1, m, &mut nums2, n);

    println!("{nums1:?}");
}

fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    for i in (m as usize)..(m as usize + n as usize) {
        nums1[i] = nums2[i - m as usize];
    }

    nums1.sort();
}