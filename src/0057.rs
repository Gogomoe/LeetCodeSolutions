use std::cmp::{min, max};

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        let mut i = 0;
        while i < intervals.len() && intervals[i][1] < new_interval[0] {
            result.push(intervals[i].clone());
            i += 1;
        }

        while i < intervals.len() && intervals[i][0] <= new_interval[1] {
            new_interval[0] = min(new_interval[0], intervals[i][0]);
            new_interval[1] = max(new_interval[1], intervals[i][1]);
            i += 1;
        }
        result.push(new_interval);

        while i < intervals.len() {
            result.push(intervals[i].clone());
            i += 1;
        }

        return result;
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]));
    // [[1,5],[6,9]]
    println!("{:?}", Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![4, 8]));
    // [[1,2],[3,10],[12,16]]
    println!("{:?}", Solution::insert(vec![vec![1, 5]], vec![2, 3]));
    // [[1,5]]
    println!("{:?}", Solution::insert(vec![vec![1, 5]], vec![2, 7]));
    // [[1,7]]
}