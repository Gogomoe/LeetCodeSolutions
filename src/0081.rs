impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        return search_in_range(0, nums.len() - 1, &nums, target);
    }
}

fn search_in_range(start: usize, end: usize, nums: &Vec<i32>, target: i32) -> bool {
    if end - start <= 1 {
        return nums[start] == target || nums[end] == target;
    }

    let mid = (start + end) / 2;
    if nums[mid] < nums[end] {
        return if nums[mid] <= target && target <= nums[end] {
            search_in_range(mid, end, nums, target)
        } else {
            search_in_range(start, mid - 1, nums, target)
        };
    }
    if nums[mid] > nums[start] {
        return if nums[start] <= target && target <= nums[mid] {
            search_in_range(start, mid, nums, target)
        } else {
            search_in_range(mid + 1, end, nums, target)
        };
    }

    return search_in_range(start, mid - 1, nums, target) || search_in_range(mid, end, nums, target);
}

struct Solution {}

fn main() {
    println!("{}", Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0)); // true
    println!("{}", Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3)); // false
}