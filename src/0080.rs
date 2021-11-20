impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut number = nums[0];
        let mut count = 1;
        let mut pos = 1;
        let mut delete = 0;
        for i in 1..nums.len() {
            if nums[i] == number {
                count += 1;
            } else {
                number = nums[i];
                count = 1;
            }
            if count <= 2 {
                nums[pos] = number;
                pos += 1;
            } else {
                delete += 1;
            }
        }
        return (nums.len() - delete) as i32;
    }
}

struct Solution {}

fn main() {
    let mut vec1 = vec![1, 1, 1, 2, 2, 3];
    println!("{}", Solution::remove_duplicates(&mut vec1)); // 5
    println!("{:?}", vec1); // [1,1,2,2,3,_]
    let mut vec2 = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
    println!("{}", Solution::remove_duplicates(&mut vec2)); // 7
    println!("{:?}", vec2); // [0,0,1,1,2,3,3,_,_]
}