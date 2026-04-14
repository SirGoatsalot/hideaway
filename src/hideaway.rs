// Core library for the Hideaway backup tool.
//
// Author: Foster Sullivan (fostersullivan12@gmail.com)

use std::fs::{DirEntry, read_dir, File, write};
use std::path::Path;
use std::io::{self, Read};

use rkyv::{rancor::Error, deserialize};

#[doc(inline)]
use crate::trove::Trove;

// Traverse the given directory, and at each file:
//      - Hash the file
//      - if hash_dict does not include new_hash
//          - if hash_dict contains an entry for the file's path
//              - remove old_key/hash combo
//          - store new_hash in hash directory (path : hash)
//          - Create backup of the file (at path relative to backup_path)
pub fn hide(root: &String, backup: &String) {
    println!("Hiding away {root} at {backup}");
    
    let mut trove = Trove::new(backup);

    let mut hash_files = | path: &DirEntry |  {
        let path_str = path.path().into_os_string().into_string();
        let filename = path_str.unwrap();

        let mut file = match File::open(&filename) {
            Err(e) => panic!("Couldn't open file {filename}: {e}"),
            Ok(file) => file,
        };
        
        // Read the current file into a buffer, and hash it
        let mut buf: Vec<u8> = vec![];
        match file.read_to_end(&mut buf) {
            Err(e) => panic!("Couldn't read file {filename}: {e}"),
            Ok(n) => println!("{filename}: {n} bytes read"),
        };

        let need_backup = trove.update(filename.to_string(), &buf);  
        if need_backup { 
            match backup_file(&filename, &backup, buf) {
                Err(e) => println!("Couldn't backup file: {e}"),
                Ok(()) => {},
            }; 
        }
        
        println!("");
    };

    match traverse(Path::new(root), &mut hash_files) {
       Ok(()) => {},
       Err(e) => println!("Error reading directory: {e}"),
    }

}

/// Traverse the given Path, and call the closure func on each file entry. 
fn traverse<F: FnMut(&DirEntry)>(dir: &Path, func: &mut F) -> io::Result<()> {
    let path = Path::new(dir);
    if path.is_dir() {
        for entry in read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                traverse(&path, func)?;
            } else {
                func(&entry);
            }
        }
    }
    Ok(())
}

/// Make a copy of the file into the backup directory
fn backup_file(filename: &String, backup: &String, data: Vec<u8>) -> io::Result<()> {
    println!("Backing up {filename}");
        
    Ok(())
}
