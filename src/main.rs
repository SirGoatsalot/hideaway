mod hideaway;
mod trove;
mod cli;

use std::env;

fn main() {
    let _args: Vec<String> = env::args().collect();
    // hideaway::hide(&args[1], &args[2]);
    hideaway::hide(&String::from("/home/copepod/test/realdir"), &String::from("/home/copepod/test/backupdir"));
}
