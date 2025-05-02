use core::fmt;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ScanArgs {

    #[arg(short, long)]
    path: String

}

impl fmt::Display for ScanArgs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Given Path {}", self.path)
    }
}