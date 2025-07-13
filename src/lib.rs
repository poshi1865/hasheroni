// Things to remember: 
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

#![allow(dead_code)]

use std::hash::{DefaultHasher, Hash, Hasher};
use std::fmt::Display;

const CAPACITY: usize = 8192;

struct Cell<K, V> {
    key: K,
    value: V
}

pub struct Hasheroni<K, V> {
    size: usize,
    table: [Option<Cell<K, V>>; CAPACITY]
}

fn get_initial_index<T: Hash>(k: &T) -> usize {
    let mut hasher = DefaultHasher::new();
    k.hash(&mut hasher);
    let initial_index = (hasher.finish() % CAPACITY as u64) as usize;
    initial_index
}

impl<K: Hash+Display+PartialEq, V: Display+Clone> Hasheroni<K, V> {
    pub fn new() -> Hasheroni<K, V> {
        Hasheroni { size: 0, table: [const{None}; CAPACITY] }
    }

    pub fn from(arr: Vec<(K, V)>) -> Hasheroni<K, V> {
        let mut hashmap: Hasheroni<K, V> = Hasheroni {
            size: 0,
            table: [const {None}; CAPACITY]
        }; 

        //array might not allow move
        for (key, value) in arr {
            hashmap.insert(key, value);
        }
        hashmap
    }

    pub fn insert(&mut self, key: K, value: V) -> Result<(), String> {
        if self.size >= CAPACITY {
            return Err("Hash table full!".to_string());
        }

        // Find index and insert
        let mut initial_index = get_initial_index(&key);

        // Linear probe and store
        for _ in 0..CAPACITY {
            match self.table[initial_index] {
                Some(_) => {
                    initial_index += 1;
                    initial_index = initial_index % CAPACITY;
                },
                None => {
                    let new_cell = Cell {key: key, value: value};
                    self.table[initial_index] = Some(new_cell);
                    self.size += 1;
                    return Ok(());
                }
            }
        }
        panic!("Could not find place to insert, and capacity has still not been met.")
    }

    pub fn get(&self, key: K) -> Option<V> {
        let mut initial_index = get_initial_index(&key);

        for _ in 0..CAPACITY {
            match &self.table[initial_index] {
                Some(cell) => {
                    if cell.key == key {
                        return Some(cell.value.clone());
                    } 
                    else {
                        initial_index += 1;
                        initial_index = initial_index % CAPACITY;
                    }
                } 
                None => return None,
            }
        }
        None
    }

    pub fn remove(&mut self, key: K) -> Result<(), String> {
        let mut initial_index = get_initial_index(&key);

        for _ in 0..CAPACITY {
            match &self.table[initial_index] {
                Some(cell) => {
                    if cell.key == key {
                        self.table[initial_index] = None;
                        self.size -= 1;
                        return Ok(());
                    } 
                    else {
                        initial_index += 1;
                        initial_index = initial_index % CAPACITY;
                    }
                } 
                None => return Err("Key does not exist.".to_string()),
            }
        }
        Err("Key does not exist.".to_string())
    }

    fn size(&self) -> usize {
        self.size
    }

    fn print(&self) {
        let mut none_count = 0;

        for (ind, item) in self.table.iter().enumerate() {
            match item {
                Some(cell) => {
                    println!("{} ::: {} ::: {}", ind, cell.key, cell.value);
                    none_count = 0;
                },
                None =>  {
                    if none_count == 3 {
                        println!("...");
                        none_count += 1;
                        continue
                    }
                    else if none_count > 3 {
                        continue
                    }
                    println!("{} ::::::", ind);
                    none_count += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let mut hm: Hasheroni<u32, String> = Hasheroni::new();
        hm.insert(10, "Ten".to_string());
        hm.insert(1, "One".to_string());
        hm.insert(9, "Nine".to_string());

        assert_eq!(hm.size(), 3);

        assert_eq!(hm.get(10), Some("Ten".to_string()));
        assert_eq!(hm.get(1), Some("One".to_string()));
        assert_eq!(hm.get(9), Some("Nine".to_string()));
        assert_eq!(hm.get(76), None);

        assert_eq!(hm.size(), 3);
    }

    #[test]
    fn test_insert() {
        let mut hm: Hasheroni<usize, String> = Hasheroni::new();

        for i in 0..CAPACITY {
            let result: Result<(), String> = hm.insert(i, "value".to_string());
            assert_eq!(result, Ok(()));
        }

        assert_eq!(hm.get(10), Some("value".to_string()));

        assert_eq!(
            hm.insert(10, "hello".to_string()),
            Err("Hash table full!".to_string())
        );

        assert_eq!(hm.size(), 8192);
    }

    #[test]
    fn test_remove() {
        let mut hm: Hasheroni<u32, String> = Hasheroni::new();
        hm.insert(10, "Ten".to_string());
        hm.insert(1, "One".to_string());
        hm.insert(9, "Nine".to_string());

        assert_eq!(hm.get(10), Some("Ten".to_string()));

        assert_eq!(hm.remove(10), Ok(()));
        assert_eq!(hm.remove(10), Err("Key does not exist.".to_string()));

        assert_eq!(hm.size(), 2);
    }

    #[test]
    fn test_from() {
        let vector: Vec<(u32, String)> = vec![
            (101, String::from("Nice")),
            (2, String::from("Plants")),
            (7, String::from("Guitar"))
        ];
        let mut hm: Hasheroni<u32, String> = Hasheroni::from(vector);

        assert_eq!(hm.get(101), Some("Nice".to_string()));
        assert_eq!(hm.get(2), Some("Plants".to_string()));
        assert_eq!(hm.get(7), Some("Guitar".to_string()));
        assert_eq!(hm.get(76), None);

        assert_eq!(hm.remove(101), Ok(()));
        assert_eq!(hm.remove(101), Err("Key does not exist.".to_string()));

        assert_eq!(hm.size(), 2);
    }
}
