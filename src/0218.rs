use std::cmp::max;

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        return merge(&buildings, 0, buildings.len() - 1);
    }
}

fn merge(buildings: &Vec<Vec<i32>>, start: usize, end: usize) -> Vec<Vec<i32>> {
    if start == end {
        let building = &buildings[start];
        return vec![vec![building[0], building[2]], vec![building[1], 0]];
    }

    let mid = (start + end) / 2;
    let left = merge(buildings, start, mid);
    let right = merge(buildings, mid + 1, end);

    let mut l = 0;
    let mut r = 0;
    let mut lh = 0;
    let mut rh = 0;
    let mut x;
    let mut h = 0;
    let mut result = Vec::new();
    while l < left.len() && r < right.len() {
        if left[l][0] < right[r][0] {
            x = left[l][0];
            lh = left[l][1];
            l += 1;
        } else if right[r][0] < left[l][0] {
            x = right[r][0];
            rh = right[r][1];
            r += 1;
        } else {
            x = left[l][0];
            lh = left[l][1];
            rh = right[r][1];
            l += 1;
            r += 1;
        }
        let new_h = max(lh, rh);
        if new_h != h {
            h = new_h;
            result.push(vec![x, h]);
        }
    }

    while l != left.len() {
        result.push(vec![left[l][0], left[l][1]]);
        l += 1;
    }
    while r != right.len() {
        result.push(vec![right[r][0], right[r][1]]);
        r += 1;
    }

    return result;
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::get_skyline(vec![vec![2, 9, 10], vec![3, 7, 15], vec![5, 12, 12], vec![15, 20, 10], vec![19, 24, 8]]));
    // [[2,10],[3,15],[7,12],[12,0],[15,10],[20,8],[24,0]]
    println!("{:?}", Solution::get_skyline(vec![vec![0, 2, 3], vec![2, 5, 3]]));
    // [[0,3],[5,0]]
}