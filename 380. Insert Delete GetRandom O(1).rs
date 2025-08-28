use rand::{thread_rng, prelude::IteratorRandom}; 
use std::collections::HashMap;

struct RandomizedSet {
    // Necessary for O(1) insert and remove
    set: HashMap<i32, usize>,
    // Necessary for O() random access
    values: Vec<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            set: HashMap::new(),
            values: Vec::new(),
        } 
    }
    
    fn insert(&mut self, val: i32) -> bool {
        // We have to check if val is already present
        if self.set.contains_key(&val) {
            return false;
        }

        // Insert val into HashMap and append it to Vector
        self.set.insert(val, self.values.len());
        self.values.push(val);

        true
    }
    
    fn remove(&mut self, val: i32) -> bool {
        let i = self.set.remove(&val);

        // Does val even exists?
        if i.is_none() {
            return false;
        }

        // Get index of to be removed element
        let i = i.unwrap();

        // Swap element at index i with last index to avoid
        // otherwise necessary shift of elements.
        // This changes the order inside the vector, which 
        // is irrelevant for this problem, as the index is 
        // tracked inside the HashMap for O(1) access anyways
        self.values.swap_remove(i);
        
        // We have to update the index of the swapped element.
        // However, this is not necessary if val was positioned
        // at the last index.
        if i < self.values.len() {
            self.set.insert(self.values[i], i);
        }

        true
    }
    
    fn get_random(&self) -> i32 {
        *self.values.iter().choose(&mut thread_rng()).unwrap()
    }
}
