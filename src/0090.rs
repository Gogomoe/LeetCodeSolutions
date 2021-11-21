impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut result = Vec::new();
        dfs(0, &mut Vec::new(), &nums, &mut result);
        return result;
    }
}

fn dfs(pos: usize, list: &mut Vec<i32>, nums: &Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if pos == nums.len() {
        result.push(list.clone());
        return;
    }
    let next = find_next_distinct(pos, nums);
    dfs(next, list, nums, result);
    for i in pos..next {
        list.push(nums[i]);
        dfs(next, list, nums, result);
    }
    for _ in pos..next {
        list.pop();
    }
}

fn find_next_distinct(mut i: usize, arr: &Vec<i32>) -> usize {
    let value = arr[i];
    while i < arr.len() && arr[i] == value {
        i += 1;
    }
    return i;
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::subsets_with_dup(vec![1, 2, 2])); // [[],[1],[1,2],[1,2,2],[2],[2,2]]
    println!("{:?}", Solution::subsets_with_dup(vec![0])); // [[],[0]]
    println!("{:?}", Solution::subsets_with_dup(vec![4, 4, 4, 1, 4])); // [[],[1],[1,4],[1,4,4],[1,4,4,4],[1,4,4,4,4],[4],[4,4],[4,4,4],[4,4,4,4]]
}