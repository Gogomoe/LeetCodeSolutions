use std::cmp::{min, max};

impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            let tmp = nums1;
            nums1 = nums2;
            nums2 = tmp;
        }

        let size = nums1.len() + nums2.len();

        let mut L = 0;
        let mut R = nums1.len();

        while L < R {
            let M = (L + R + 1) / 2;
            let N = size / 2 - M;

            if M == nums1.len() {
                if *nums1.last().unwrap_or(&i32::min_value()) <= nums2[N] {
                    L = M;
                    break;
                } else {
                    R = M - 1;
                    continue;
                }
            }

            if nums1[M] < nums2[N - 1] {
                L = M;
            } else if nums1[M - 1] > nums2[N] {
                R = M - 1;
            } else {
                L = M;
                break;
            }
        }
        R = size / 2 - L;
        return if size % 2 != 0 {
            min(
                *nums1.get(L).unwrap_or(&i32::max_value()),
                *nums2.get(R).unwrap_or(&i32::max_value()),
            ) as f64
        } else {
            let max_of_left = max(
                if L == 0 || nums1.len() == 0 { i32::min_value() } else { nums1[L - 1] },
                if R == 0 || nums2.len() == 0 { i32::min_value() } else { nums2[R - 1] },
            ) as f64;
            let min_of_right = min(
                *nums1.get(L).unwrap_or(&i32::max_value()),
                *nums2.get(R).unwrap_or(&i32::max_value()),
            ) as f64;
            return (max_of_left + min_of_right) / 2.0;
        };
    }
}

struct Solution {}

fn main() {
    println!(
        "{}",
        Solution::find_median_sorted_arrays(
            vec![1, 3],
            vec![2],
        )
    );
    println!(
        "{}",
        Solution::find_median_sorted_arrays(
            vec![1, 2],
            vec![3, 4],
        )
    );
    println!(
        "{}",
        Solution::find_median_sorted_arrays(
            vec![0, 0],
            vec![0, 0],
        )
    );
    println!(
        "{}",
        Solution::find_median_sorted_arrays(
            vec![],
            vec![1],
        )
    );
    println!(
        "{}",
        Solution::find_median_sorted_arrays(
            vec![2],
            vec![],
        )
    );
    println!(
        "{}",
        Solution::find_median_sorted_arrays(
            vec![2],
            vec![1, 3, 4, 5],
        )
    );
}