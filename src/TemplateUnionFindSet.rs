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