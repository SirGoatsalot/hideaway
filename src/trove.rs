// Structure defining the storage of Hideaway backup hash data.
//
// Auhtor: Foster Sullivan (fostersullivan12@gmail.com)

use std::collections::HashMap;   
use blake3::{Hash, hash};
use rkyv::{Archive, Serialize, Deserialize, rancor::Error};

pub const TROVES_DIR: &str = "/home/copepod/.local/share/hideaway/";

#[derive(Serialize, Deserialize, Archive)]
pub struct Trove {
    backup_dir: String,
    root: String,
    map: HashMap<String, [u8; 32]>,
}

/** Struct holding the hash comparison logic and file writing logic for Hideaway.
 */
impl Trove {
    /** Attempts to load a `.trove` file from `~/.local/share/hideaway`  
     * If no file is found, a new `.trove` file is created.
    */
    pub fn new(backup_dir: &String, root: &String) -> Self {
        let filename = Trove::filename_from(backup_dir);
        let filepath = TROVES_DIR.to_owned() + &filename;
        println!("Searching for Trove at {filepath}");
        match std::fs::read(filepath) {
            Ok(file) => {
                match rkyv::access::<ArchivedTrove, Error>(file.as_slice()) {
                    Ok(archive) => { 
                        println!("Found Trove!");
                        match rkyv::deserialize::<Trove, Error>(archive) {
                            Ok(trove) => trove,
                            Err(e) => panic!("Unable to deserialize Archived Trove: {e}"),
                        } 
                    },
                    Err(e) => panic!("Unable to access Archived Trove: {e}"),
                }
            },
            Err(_) => {
                println!("No Trove found, making new Trove");
                let map: HashMap<String, [u8; 32]> = HashMap::new();
                Self {
                    backup_dir: backup_dir.to_string(),
                    root: root.to_string(),
                    map: map,
                }
            },
        }
    }

    pub fn _get(&self, dir: &String) -> Option<&[u8; 32]> {
        self.map.get(dir)
    }

    // Check if the file with the given name exists in the table
    pub fn update(&mut self, filename: String,  buf: &Vec<u8>) -> bool {
        let new_hash = hash(buf.as_slice());
        println!("{new_hash}");

        let need_update = match self.map.get(&filename) {
            Some(old_hash) => Hash::from_slice(old_hash).expect("woops") != new_hash, 
            None => true,
        }; 

        if need_update {
            println!("Updating {filename}");
            self.map.insert(filename, *new_hash.as_bytes());
        }

        need_update
    }

    pub fn save(&self) {
        let filename = Trove::filename_from(&self.backup_dir);
        let filepath = TROVES_DIR.to_owned() + &filename; 
        println!("Saving trove to {filepath}");
        let self_bytes = rkyv::to_bytes::<Error>(self).unwrap();
        match std::fs::write(filepath, self_bytes){
            Ok(()) => {println!("Trove saved!")},
            Err(e) => panic!("Unable to write archive to file: {e}"),
        };

    }

    fn filename_from(dir: &String) -> String {
        let filename = dir.split(std::path::MAIN_SEPARATOR);
        filename.last().unwrap().to_owned() + ".trove"
    }
}

