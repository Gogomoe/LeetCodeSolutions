use std::iter;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut used: Vec<bool> = iter::repeat(false).take(nums.len()).collect();
        let mut list: Vec<i32> = Vec::new();
        let mut result = Vec::new();
        dfs(0, &mut list, &mut used, &nums, &mut result);
        return result;
    }
}

fn dfs(count: usize, list: &mut Vec<i32>, used: &mut Vec<bool>, nums: &Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if count == nums.len() {
        result.push(list.clone());
        return;
    }
    for i in 0..nums.len() {
        if used[i] { continue; }
        used[i] = true;
        list.push(nums[i]);
        dfs(count + 1, list, used, nums, result);
        list.pop();
        used[i] = false;
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::permute(vec![1, 2, 3])); // [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
    println!("{:?}", Solution::permute(vec![0, 1])); // [[0,1],[1,0]]
    println!("{:?}", Solution::permute(vec![1])); // [[1]]
}