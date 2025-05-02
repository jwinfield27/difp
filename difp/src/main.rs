use clap::Parser;

mod cli;
use cli::scan_args::ScanArgs;

fn main() {
    println!("Hello, world!");
    let args = ScanArgs::parse();
    println!("{}", args);
}
