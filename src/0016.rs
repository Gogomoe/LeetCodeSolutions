impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();

        fn find_next_distinct(mut i: usize, nums: &Vec<i32>) -> usize {
            let value = nums[i];
            while i < nums.len() && nums[i] == value {
                i += 1;
            }
            return i;
        }
        fn find_prev_distinct(mut i: i32, nums: &Vec<i32>) -> i32 {
            let value = nums[i as usize];
            while i >= 0 && nums[i as usize] == value {
                i -= 1;
            }
            return i;
        }
        let mut result = nums[0] + nums[1] + nums[2];
        let mut i = 0;
        while i < nums.len() - 2 {
            let mut j = i + 1;
            let mut k = (nums.len() - 1) as i32;
            while (j as i32) < k {
                let sum = nums[i] + nums[j] + nums[k as usize];
                if sum > target {
                    if (sum - target).abs() < (result - target).abs() {
                        result = sum;
                    }
                    k = find_prev_distinct(k, &nums);
                } else if sum < target {
                    if (sum - target).abs() < (result - target).abs() {
                        result = sum;
                    }
                    j = find_next_distinct(j, &nums);
                } else {
                    return target;
                }
            }
            i = find_next_distinct(i, &nums);
        }

        result
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
    println!("{:?}", Solution::three_sum_closest(vec![0, 0, 0], 1));
}