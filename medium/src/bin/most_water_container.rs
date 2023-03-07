fn main() {
    let height: Vec<i32> = vec![1,8,6,2,5,4,8,3,7];

    // let mut height: Vec<i32> = vec![];

    // for i in 0..10001 {
    //     height.push(i);
    // }

    println!("Max area: {}", max_area(height));
}

fn max_area(height: Vec<i32>) -> i32 {
    use std::cmp::{min, max};
    
    let mut left = 0;
    let mut right = height.len() - 1;

    let mut area = 0;

    while left < right {
        area = max(area, (right - left) as i32 * min(height[left], height[right]));

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    area
}