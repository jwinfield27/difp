use core::fmt;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ScanArgs {

    #[arg(short, long, default_value_t = String::from("."))]
    path: String,

    #[arg(short, long, default_value_t = String::from("main"))]
    branch: String,

    #[arg(short, long, default_value_t = String::from("origin"))]
    remote: String
}

impl ScanArgs {
    pub fn get_path(&self) -> &String {
        &self.path
    }
}

impl fmt::Display for ScanArgs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Given Path {}", self.path)
    }
}