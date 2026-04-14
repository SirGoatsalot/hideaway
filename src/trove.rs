// Structure defining the storage of Hideaway backup hash data.
//
// Auhtor: Foster Sullivan (fostersullivan12@gmail.com)

use std::collections::HashMap;   
use blake3::{Hash, hash};

pub struct Trove {
    backup_dir: String,
    map: HashMap<String, Hash>,
}

/** Struct holding the hash comparison logic and file writing logic for Hideaway.
 */
impl Trove {
    /** Attempts to load a `.trove` file from `~/.local/share/hideaway`  
     * If no file is found, a new `.trove` file is created.
    */
    pub fn new(dir: &String) -> Self {
        let map: HashMap<String, Hash> = HashMap::new();
        Self {
            backup_dir: dir.to_string(),
            map: map,
        }
    }

    pub fn _get(&self, dir: &String) -> Option<&Hash> {
        self.map.get(dir)
    }

    // Check if the file with the given name exists in the table
    pub fn update(&mut self, filename: String,  buf: &Vec<u8>) -> bool {
        let new_hash = hash(buf.as_slice());
        println!("{new_hash}");

        let need_update = match self.map.get(&filename) {
            Some(old_hash) => old_hash != &new_hash, 
            None => true,
        }; 

        if need_update {
            println!("Updating {filename}");
            self.map.insert(filename, new_hash);
        }

        need_update
    }
}

