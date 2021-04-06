use std::iter;
use std::collections::BinaryHeap;
use std::cmp::{Ordering, max};

struct Block {
    height: i32,
    x: usize,
    y: usize,
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.height).cmp(&(other.height)).reverse()
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        (self.height) == (other.height)
    }
}

impl Eq for Block {}


impl Solution {
    pub fn trap_rain_water(mut height_map: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let m = height_map.len();
        let n = height_map[0].len();
        let mut visited: Vec<Vec<bool>> = iter::repeat(
            iter::repeat(false).take(n).collect()
        ).take(m).collect();

        let mut heap: BinaryHeap<Block> = BinaryHeap::new();

        fn push_into_heap(
            y: usize, x: usize,
            visited: &mut Vec<Vec<bool>>, heap: &mut BinaryHeap<Block>,
            height_map: &Vec<Vec<i32>>,
        ) {
            if !visited[y][x] {
                heap.push(Block {
                    height: height_map[y][x],
                    x,
                    y,
                });
                visited[y][x] = true;
            }
        }

        for i in 0..n {
            push_into_heap(0, i, &mut visited, &mut heap, &height_map);
            push_into_heap(m - 1, i, &mut visited, &mut heap, &height_map);
        }
        for i in 0..m {
            push_into_heap(i, 0, &mut visited, &mut heap, &height_map);
            push_into_heap(i, n - 1, &mut visited, &mut heap, &height_map);
        }

        while !heap.is_empty() {
            let b = heap.pop().unwrap();
            let (height, y, x) = (b.height, b.y, b.x);

            if x > 0 && !visited[y][x - 1] {
                let fill = max(height - height_map[y][x - 1], 0);
                result += fill;
                height_map[y][x - 1] += fill;
                push_into_heap(y, x - 1, &mut visited, &mut heap, &height_map);
            }
            if x < n - 1 && !visited[y][x + 1] {
                let fill = max(height - height_map[y][x + 1], 0);
                result += fill;
                height_map[y][x + 1] += fill;
                push_into_heap(y, x + 1, &mut visited, &mut heap, &height_map);
            }
            if y > 0 && !visited[y - 1][x] {
                let fill = max(height - height_map[y - 1][x], 0);
                result += fill;
                height_map[y - 1][x] += fill;
                push_into_heap(y - 1, x, &mut visited, &mut heap, &height_map);
            }
            if y < m - 1 && !visited[y + 1][x] {
                let fill = max(height - height_map[y + 1][x], 0);
                result += fill;
                height_map[y + 1][x] += fill;
                push_into_heap(y + 1, x, &mut visited, &mut heap, &height_map);
            }
        }

        result
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::trap_rain_water(
        vec![
            vec![1, 4, 3, 1, 3, 2],
            vec![3, 2, 1, 3, 2, 4],
            vec![2, 3, 3, 2, 3, 1]
        ]
    ))
}