use std::net::Shutdown::Read;

/**
 * // This is the MountainArray's API interface.
 * // You should not implement it, or speculate about its implementation
 *  struct MountainArray;
 *  impl MountainArray {
 *     fn get(index:i32)->i32;
 *     fn length()->i32;
 * };
 */


impl Solution {
    pub fn find_in_mountain_array(target: i32, mountainArr: &MountainArray) -> i32 {
        let len = mountainArr.length();
        let mut left = 1;
        let mut right = len as i32 - 2;

        while left < right {
            let mid = (left + right + 1) / 2;
            if mountainArr.get(mid - 1) < mountainArr.get(mid) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }

        let highest = left;

        left = 0;
        right = highest;
        while left < right {
            let mid = (left + right) / 2;
            if mountainArr.get(mid) >= target {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        if mountainArr.get(left) == target {
            return left;
        }

        left = highest;
        right = len - 1;
        while left < right {
            let mid = (left + right) / 2;
            if mountainArr.get(mid) <= target {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        if mountainArr.get(left) == target {
            return left;
        }

        -1
    }
}

struct MountainArray {}

impl MountainArray {
    fn get(&self, index: i32) -> i32 {
        unimplemented!();
    }
    fn length(&self) -> i32 {
        unimplemented!();
    }
}

struct Solution {}

fn main() {}