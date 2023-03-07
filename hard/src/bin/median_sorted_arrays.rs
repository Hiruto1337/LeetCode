fn main() {
    let nums1: Vec<i32> = vec![1, 3, 5];
    let nums2: Vec<i32> = vec![2, 4, 6, 7, 8, 9, 10];
    // TEST FOR TOMME ARRAYS
    // TEST FOR SAMME TAL

    println!("{}", find_median_sorted_arrays(nums1, nums2));
}

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (mut nums1, mut nums2) = (nums1, nums2);

    let even = (nums1.len() + nums2.len()) % 2 == 0;
    let disconnected = nums1[nums1.len() - 1] < nums2[0] || nums2[nums2.len() - 1] < nums1[0];

    if disconnected {
    } else {
        if even {
            while nums1.len() != 1 || nums2.len() != 1 {
                let mid1 = nums1.len() / 2;
                let mid2 = nums2.len() / 2;

                if nums1[mid1] < nums2[mid2] {
                    nums1 = nums1[mid1 + 1..].to_vec();
                    nums2 = nums2[..mid2].to_vec();
                } else if nums1[mid1] > nums2[mid2] {
                    nums2 = nums2[mid2 + 1..].to_vec();
                    nums1 = nums1[..mid1].to_vec();
                }
            }

            println!("{nums1:?}");
            println!("{nums2:?}");

            return (nums1[0] as f64 + nums2[0] as f64) / 2.0;
        } else {
        }
    }

    69.0
}

// If nums1.len() + nums2.len() is odd, a single value will be returned
// If the total length is even, an average will be returned
