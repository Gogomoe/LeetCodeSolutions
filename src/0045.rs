impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut index = 0;
        let mut jump = 0;
        while index != nums.len() - 1 {
            jump += 1;
            if index + nums[index] as usize >= nums.len() - 1 {
                break;
            }
            let mut max_target = index + nums[index] as usize;
            let mut max_index = index;
            for i in index..=index + nums[index] as usize {
                if i + nums[i] as usize > max_target {
                    max_target = i + nums[i] as usize;
                    max_index = i;
                }
            }
            index = max_index;
        }
        return jump;
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::jump(vec![2, 3, 1, 1, 4])); // 2
    println!("{}", Solution::jump(vec![2, 3, 0, 1, 4])); // 2
}