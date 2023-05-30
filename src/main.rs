// import clap parser
use clap::Parser;

#[derive(Parser)]
#[command(name = "wet")]
#[command(about = "Weather in your terminal", long_about = None)]

struct Args {
    days: u8,
}

//  Create a CLI

// Make HTTP request and parse output

fn main() {
    println!("Hello, world!");
}
