use std::cmp::{max, min};

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        if arr.len() <= 1 {
            return 0;
        }
        let mut end: usize = 0;
        let mut start: usize = arr.len() - 1;

        for i in 1..arr.len() {
            if arr[i] >= arr[i - 1] { end = i; } else { break; }
        }

        if end == arr.len() - 1 {
            return 0;
        }

        for i in (1..arr.len()).rev() {
            if arr[i] >= arr[i - 1] { start = i - 1; } else { break; }
        }

        let mut result: usize = arr.len() - max(end + 1, arr.len() - start);

        let mut i = end as i32;
        let mut j = arr.len() - 1;

        while i >= 0 && arr[i as usize] > arr[j] {
            i -= 1;
        }
        result = min(result, j - i as usize - 1);

        while j >= start {
            while i >= 0 && arr[i as usize] > arr[j] {
                i -= 1;
            }
            if i < 0 { break; }
            result = min(result, j - i as usize - 1);
            j -= 1;
        }

        return result as i32;
    }
}

struct Solution {}

fn main() {
    let res = Solution::find_length_of_shortest_subarray(Vec::from([1, 2, 3, 10, 0, 7, 8, 9]));
    println!("{}", res);
}