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
            self.parent[pa] = pb;
            self.size -= 1;
        }
    }
}

fn similar(s1: &String, s2: &String) -> bool {
    let mut diff = 0;
    let mut cs1 = s1.chars();
    let mut cs2 = s2.chars();

    while let Some(c1) = cs1.next() {
        if c1 != cs2.next().unwrap() { diff += 1 }
    }
    diff == 2 || diff == 0
}

impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let mut ufs = UnionFindSet::new(strs.len());
        for i in 0..strs.len() {
            for j in 0..i {
                if similar(&strs[i], &strs[j]) {
                    ufs.union(i, j);
                }
            }
        }
        ufs.size as i32
    }
}

struct Solution {}

fn main() {}