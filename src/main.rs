mod manager;
mod ports;
mod adapters;

use std::path::Path;
use clap::Parser;
use adapters::executor::CopyStrategy;
use crate::adapters::executor::{DeleteStrategy, ViewStrategy};
use crate::manager::{check_contains, ensure_target, FileExecutor, make_pattern};
use crate::ports::arguments::ArgsConfig;


fn main() {
    let args = ArgsConfig::parse();
    let regex = make_pattern(args.patterns);
    println!("{}", regex.as_str());
    let root_path = Path::new(&args.root);
    let copy_to = Path::new(&args.to);
    check_contains(root_path, copy_to);
    ensure_target(copy_to);
    if args.show {
        let strategy = ViewStrategy{};
        let manager = FileExecutor {root_dir_path: root_path, regex: &regex, strategy: &strategy};
        manager.act().expect("Viewing failed.");
    }
    if args.copy {
        let strategy: CopyStrategy = CopyStrategy{destination: copy_to};
        let manager = FileExecutor { root_dir_path: root_path, regex: &regex, strategy: &strategy};
        manager.act().expect("Copying failed.");
    }
    if args.rm {
        let strategy = DeleteStrategy;
        let manager = FileExecutor { root_dir_path: root_path, regex: &regex, strategy: &strategy };
        manager.act().expect("Removal failed.")
    }
}
