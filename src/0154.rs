use std::cmp::min;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        return find_min_in_range(&nums, 0, nums.len() - 1);
    }
}

fn find_min_in_range(nums: &Vec<i32>, begin: usize, end: usize) -> i32 {
    if end - begin <= 1 {
        return min(nums[begin], nums[end]);
    }

    let mut mininum = nums[begin];

    if nums[begin] >= nums[end] {
        let mid = (begin + end) / 2;
        if nums[mid] >= nums[begin] {
            mininum = min(mininum, find_min_in_range(nums, mid, end))
        }
        if nums[mid] <= nums[begin] {
            mininum = min(mininum, find_min_in_range(nums, begin, mid))
        }
    }

    mininum
}

struct Solution {}

fn main() {}