use std::cmp::Reverse;
use std::collections::BinaryHeap;

type I32R = Reverse<i32>;

struct MedianFinder {
    bigger: BinaryHeap<I32R>,
    smaller: BinaryHeap<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    /** initialize your data structure here. */
    fn new() -> Self {
        MedianFinder {
            bigger: BinaryHeap::new(),
            smaller: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.bigger.push(Reverse(num));
        let small = self.bigger.pop().unwrap().0;
        self.smaller.push(small);

        if self.smaller.len() > self.bigger.len() {
            let it = self.smaller.pop().unwrap();
            self.bigger.push(Reverse(it));
        }
    }

    fn find_median(&self) -> f64 {
        if self.smaller.len() == self.bigger.len() {
            let l = *self.smaller.peek().unwrap() as f64;
            let r = self.bigger.peek().unwrap().0 as f64;
            (l + r) / 2.0
        } else {
            self.bigger.peek().unwrap().0 as f64
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

fn main() {}