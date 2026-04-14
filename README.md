# Hideaway

Goals Stage 1:
 - Backup JDex directories (or directories matching a custom regex) manually
 - Check directories for unchanged files in old backup by checking hashes
 - Barebones CLI

Goals Stage 2:
 - Use compression for backup to save space
 - Restore the last backup into a given directory
 - use custom hash function for hash matching


## Architecture

hideaway.rs
 - Combs through directory tree recursively from given path.
 - For each file:
    - Check if backup exists of file, if not make one
    - Check if existing backup hash matches current file hash

cli.rs
 - Gets user input at the command line
 - used for future stretch goals

main.rs
 - pass directory to check to hideaway.rs
 - will orchestrate cli.rs in the future
