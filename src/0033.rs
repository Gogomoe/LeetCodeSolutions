impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums[0] <= nums[nums.len() - 1] {
            return match nums.binary_search(&target) {
                Ok(it) => it as i32,
                Err(_) => -1
            };
        }

        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r {
            if nums[l] <= nums[r] {
                r = l;
                break;
            }

            let mid = (l + r) / 2;
            if nums[mid] >= nums[l] {
                l = mid + 1;
            } else {
                r = mid;
            }
        }

        return if target <= nums[nums.len() - 1] {
            match nums[r..].binary_search(&target) {
                Ok(it) => (r + it) as i32,
                Err(_) => -1
            }
        } else {
            match nums[..r].binary_search(&target) {
                Ok(it) => it as i32,
                Err(_) => -1
            }
        };
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));// 4
    println!("{:?}", Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3));// -1
    println!("{:?}", Solution::search(vec![1], 1));// 0
}