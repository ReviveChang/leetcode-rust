use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    big:BinaryHeap<Reverse<i32>>,
    small:BinaryHeap<i32>,
}

impl MedianFinder {

    /** initialize your data structure here. */
    fn new() -> Self {
        let mut big = BinaryHeap::new();
        let mut small = BinaryHeap::new();
        big.push(Reverse(i32::MAX));
        small.push(i32::MIN);
        MedianFinder{big,small}
    }
    
    fn add_num(&mut self, num: i32) {
        if num > *self.small.peek().unwrap(){
            self.big.push(Reverse(num));
        }else{self.small.push(num);}

        if self.small.len() > self.big.len() + 1 {
            self.big.push(Reverse(self.small.pop().unwrap()));
        }else if self.small.len() < self.big.len() {
            self.small.push(self.big.pop().unwrap().0);
        }
    }
    
    fn find_median(&self) -> f64 {
        if self.small.len() == self.big.len() {
            (*self.small.peek().unwrap()+(*self.big.peek().unwrap()).0) as f64/2.0
        }else{
            *self.small.peek().unwrap() as f64
        }
    }
}
