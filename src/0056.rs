use std::cmp::max;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut result = Vec::new();
        let mut start = intervals[0][0];
        let mut end = intervals[0][1];
        for interval in intervals {
            if interval[0] <= end {
                end = max(end, interval[1])
            } else {
                result.push(vec![start, end]);
                start = interval[0];
                end = interval[1];
            }
        }
        result.push(vec![start, end]);
        return result;
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]));
    // [[1,6],[8,10],[15,18]]
    println!("{:?}", Solution::merge(vec![vec![1, 4], vec![4, 5]]));
    // [[1,5]]
}