// import clap parser
use clap::Parser;

// define latitude and longitude

const LATITUDE: f64 = 46.83;
const LONGITUDE: f64 = -96.83;

#[derive(Parser)]
#[command(name = "forecast")]
#[command(about = "Weather in your terminal", long_about = None)]

struct Args {
    #[arg(short, default_value_t = 0)]
    days: u8,
}

//  Create a CLI

// Make HTTP request and parse output

fn main() {
    let args = Args::parse();

    println!("{:?}", args.days);
}
