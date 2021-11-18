impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut list = Vec::new();
        dfs(0, &mut list, &nums, &mut result);
        return result;
    }
}

fn dfs(i: usize, list: &mut Vec<i32>, nums: &Vec<i32>, result: &mut Vec<Vec<i32>>) {
    result.push(list.clone());
    for j in i..nums.len() {
        list.push(nums[j]);
        dfs(j + 1, list, nums, result);
        list.pop();
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::subsets(vec![1, 2, 3]));  // [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
    println!("{:?}", Solution::subsets(vec![0]));  // [[],[0]]
}