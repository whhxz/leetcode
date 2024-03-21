// Definition for singly-linked list.
#![allow(dead_code, unused_variables)]
use std::collections::HashMap;
struct FrequencyTracker {
    data: HashMap<i32, i32>,
    val_nums: HashMap<i32, i32>,
}

impl FrequencyTracker {
    fn new() -> Self {
        FrequencyTracker {
            data: HashMap::new(),
            val_nums: HashMap::new(),
        }
    }

    fn add(&mut self, number: i32) {
        let val = self.data.entry(number).or_insert(0);
        if *val != 0 {
            *self.val_nums.get_mut(val).unwrap() -= 1;
        }
        *val += 1;
        *self.val_nums.entry(*val).or_insert(0) += 1;
    }

    fn delete_one(&mut self, number: i32) {
        let val = self.data.get_mut(&number);
        if val.is_none() {
            return;
        }
        let val = val.unwrap();
        *self.val_nums.get_mut(val).unwrap() -= 1;
        if *val == 1 {
            self.data.remove(&number);
        } else {
            *val -= 1;
            *self.val_nums.entry(*val).or_insert(0) += 1;
        }
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        let num = self.val_nums.get(&frequency);
        return num.is_some() && *num.unwrap() != 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::FrequencyTracker;

    #[test]
    fn test1() {
        let mut t = FrequencyTracker::new();
        t.add(3);
        t.add(1);
        t.add(3);
        t.add(1);
        t.delete_one(10);
        t.delete_one(6);
        t.add(7);
        t.delete_one(10);
        t.add(1);
        t.add(2);
        t.add(3);
        t.has_frequency(1);
        t.delete_one(3);
        let res = t.has_frequency(2);
        assert!(res);
        let res = t.has_frequency(3);
        assert!(res);
    }
}
fn main() {}
