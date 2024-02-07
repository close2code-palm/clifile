mod manager;
mod ports;
mod adapters;

use std::path::Path;
use clap::Parser;
use crate::manager::{CopyStrategy, FileExecutor, make_pattern};
use crate::ports::arguments::ArgsConfig;


//todo make target dir, check if directories intersect
fn main() {
    let args = ArgsConfig::parse();
    let regex = make_pattern(args.patterns);
    println!("{}", regex.as_str());
    let root_path = Path::new(&args.root);
    let copy_to = Path::new(&args.to);
    if args.copy {
        let strategy: CopyStrategy = CopyStrategy{destination: copy_to};
        let manager = FileExecutor { root_dir_path: root_path, regex, strategy: &strategy};
        manager.act().expect("Copying failed");
    }
}
