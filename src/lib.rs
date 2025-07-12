// Things to remember: 
// Linear probing impl
// Dynamically resize on load factor increase

// API
//
// let a: <String, u32> = Hasheroni::new()
// let a = Hasheroni::from([(), (), ()])
// a.insert(key, value);
// contains_key(key) -> bool;
// get(key) -> value;
// remove(key);

// TODO: 
// Currently only supports strings and ints. Add support for all complex types as well.
// Implement closed addressing.
// Implement my own hash function.

// Steps
// First define a hash function

// When a hashmap is init, we initialize an array with CAPACITY

use std::hash::{DefaultHasher, Hash, Hasher};
use std::fmt::Display;

const CAPACITY: usize = 8192;

struct Cell<K, V> {
    key: K,
    value: V
}

struct Hasheroni<K, V> {
    size: usize,
    table: [Option<Cell<K, V>>; CAPACITY]
}

impl<K: Hash+Display, V: Display> Hasheroni<K, V> {
    fn new() -> Hasheroni<K, V> {
        Hasheroni { size: 0, table: [const{None}; CAPACITY] }
    }

    // fn from(arr: Vec<(K, V)>) -> Hasheroni<K, V> {
    //     let mut hashmap: Hasheroni<K, V> = Hasheroni {
    //         size: arr.len(),
    //         table: [const {None}; CAPACITY]
    //     }; 
    //
    //     //array might not allow move
    //     for (key, value) in arr {
    //         hashmap.insert(key, value);
    //     }
    //     hashmap
    // }

    fn insert(&mut self, key: K, value: V) {
        if self.size == CAPACITY {
            panic!("Hash table full!");
        }

        // Find index and insert
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let mut initial_index = (hasher.finish() % CAPACITY as u64) as usize;

        for i in [0..CAPACITY] {
            match self.table[initial_index] {
                Some(_) => {
                    initial_index += 1;
                    initial_index = initial_index % CAPACITY;
                },
                None => {
                    let new_cell = Cell {key: key, value: value};
                    self.table[initial_index] = Some(new_cell);
                    return;
                }
            }
        }
        panic!("Could not find place to insert");
    }

    fn contains_key(&self, key: K) -> bool {
        return false;
    }

    fn get(&self, key: K) -> K {
        key
    }

    fn remove(&self, key: K) {
    }

    fn print(&self) {
        for item in &self.table {
            match item {
                Some(cell) => {
                    println!("{} ::: {}", cell.key, cell.value);
                },
                None => continue
            }
        }
    }
}

pub fn test() {
    let mut hm: Hasheroni<u32, String> = Hasheroni::new();
    hm.insert(10, "Hello World!".to_string());
    hm.print();
}
