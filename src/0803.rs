use std::collections::VecDeque;
use std::iter;
use std::cmp::min;

struct UnionFindSet {
    parent: Vec<usize>,
    size: usize,
}

impl UnionFindSet {
    pub fn new(size: usize) -> UnionFindSet {
        let mut vec = Vec::with_capacity(size);
        for i in 0..size {
            vec.push(i)
        }
        UnionFindSet { parent: vec, size }
    }

    pub fn find(&mut self, it: usize) -> usize {
        if self.parent[it] != it {
            self.parent[it] = self.find(self.parent[it]);
        }
        self.parent[it]
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let pa = self.find(a);
        let pb = self.find(b);
        if pa != pb {
            self.parent[pb] = pa;
            self.size -= 1;
        }
    }
}

impl Solution {
    pub fn hit_bricks(mut grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();

        let mut result: Vec<i32> = iter::repeat(-1).take(hits.len()).collect();

        for (i, hit) in hits.iter().enumerate() {
            let y = hit[0] as usize;
            let x = hit[1] as usize;
            if grid[y][x] == 0 {
                result[i] = 0;
            }
            grid[y][x] = 0;
        }

        let mut ufs = UnionFindSet::new(m * n);
        let mut queue = VecDeque::new();
        let mut visited: Vec<Vec<bool>> = iter::repeat(
            iter::repeat(false).take(n).collect()
        ).take(m).collect();

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 || visited[i][j] { continue; }

                queue.push_back((i, j));
                visited[i][j] = true;

                while !queue.is_empty() {
                    let (y, x) = queue.pop_front().unwrap();

                    for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                        let (ny, nx) = (y as i32 + *dy, x as i32 + *dx);
                        if ny < 0 || ny >= m as i32 || nx < 0 || nx >= n as i32 { continue; }
                        let (ny, nx) = (ny as usize, nx as usize);

                        if grid[ny][nx] == 1 && !visited[ny][nx] {
                            queue.push_back((ny, nx));
                            visited[ny][nx] = true;

                            ufs.union(i * n + j, ny * n + nx);
                        }
                    }
                }
            }
        }

        let mut counts: Vec<usize> = iter::repeat(0).take(m * n).collect();
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] != 0 {
                    counts[ufs.find(i * n + j)] += 1;
                }
            }
        }

        for i in (0..hits.len()).rev() {
            if result[i] != -1 { continue; }
            result[i] = 0;
            let y = hits[i][0] as usize;
            let x = hits[i][1] as usize;

            let mut min_parent = y * n + x;

            for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                let (ny, nx) = (y as i32 + *dy, x as i32 + *dx);
                if ny < 0 || ny >= m as i32 || nx < 0 || nx >= n as i32 { continue; }
                let (ny, nx) = (ny as usize, nx as usize);

                if grid[ny][nx] == 0 { continue; }
                min_parent = min(min_parent, ufs.find(ny * n + nx));
            }
            for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                let (ny, nx) = (y as i32 + *dy, x as i32 + *dx);
                if ny < 0 || ny >= m as i32 || nx < 0 || nx >= n as i32 { continue; }
                let (ny, nx) = (ny as usize, nx as usize);

                if grid[ny][nx] == 0 { continue; }
                let parent = ufs.find(ny * n + nx);
                if parent == min_parent { continue; }
                if parent >= n && min_parent < n {
                    result[i] += counts[parent] as i32;
                }
                ufs.union(min_parent, parent);

                counts[min_parent] += counts[parent];
                counts[parent] = 0;
            }

            ufs.union(min_parent, y * n + x);
            counts[min_parent] += 1;
            grid[y][x] = 1;
        }

        result
    }
}

struct Solution {}

fn main() {
    let grid = [
        [1, 0, 0, 0],
        [1, 1, 1, 0]
    ];
    let hits = [
        [1, 0]
    ];

    let result = Solution::hit_bricks(
        grid.iter().map(|it| it.to_vec()).collect(),
        hits.iter().map(|it| it.to_vec()).collect(),
    );

    println!("{:?}", result);
}