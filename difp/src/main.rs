use clap::Parser;

mod file_io;
use file_io::discovery::{get_files_and_directories, is_directory, is_file};
mod cli;
use cli::scan_args::ScanArgs;

fn main() {
    let args = ScanArgs::parse();
    if is_directory(args.get_path()) {
        get_files_and_directories(args.get_path());
    } else if is_file(args.get_path()) {
       println!("{} is a file", args.get_path()); 
    } {
       println!("some error must've snuck through");
    }
}
