mod hideaway;
mod cli;

fn main() {
    println!("Hello, world!");
    cli::interface();
    hideaway::hide();
}
