impl Solution {
    pub fn makesquare(nums: Vec<i32>) -> bool {
        if nums.len() < 4 { return false; }
        let sum: i32 = nums.iter().sum();
        if sum % 4 != 0 { return false; }
        let part = sum / 4;
        return dfs(&mut [0; 4], 0, &nums, part);
    }
}

fn dfs(sums: &mut [i32; 4], i: usize, nums: &Vec<i32>, part: i32) -> bool {
    if i == nums.len() { return true; }
    for group in 0..4 {
        sums[group] += nums[i];
        if sums[group] <= part {
            if dfs(sums, i + 1, nums, part) {
                return true;
            }
        }
        sums[group] -= nums[i];
    }
    return false;
}

struct Solution {}

fn main() {}