impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            let mut it = nums[i];
            while it != i as i32 + 1 && it >= 1 && it <= nums.len() as i32 {
                if nums[it as usize - 1] != it {
                    nums[i] = nums[it as usize - 1];
                    nums[it as usize - 1] = it;
                } else {
                    break;
                }
                it = nums[i];
            }
        }

        for i in 0..nums.len() {
            if nums[i] != i as i32 + 1 {
                return i as i32 + 1;
            }
        }

        return nums.len() as i32 + 1;
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::first_missing_positive(vec![1, 2, 0])); // 3
    println!("{:?}", Solution::first_missing_positive(vec![3, 4, -1, 1]));// 2
    println!("{:?}", Solution::first_missing_positive(vec![7, 8, 9, 11, 12])); // 1
    println!("{:?}", Solution::first_missing_positive(vec![1])); // 2
}