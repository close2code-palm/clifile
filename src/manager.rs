use std::fs::{DirEntry, read_dir};
use std::{io};
use std::path::Path;
use regex::Regex;



pub fn make_pattern(vec: Vec<String>) -> Regex {
    Regex::new(&*vec.join("|")).unwrap()
}


pub trait ManagementStrategy {
    // fn new<'c>(destination: &'c Path) -> Self where Self: Sized;
    fn execute(&self, de:&DirEntry) -> io::Result<()>;
}

pub struct CopyStrategy<'a> {
    pub destination: &'a Path
}


pub struct FileExecutor<'a> {
    pub root_dir_path: &'a Path,
    pub regex: Regex,
    pub strategy: &'a dyn ManagementStrategy
}

impl FileExecutor<'_> {

    fn look_for_files(&self, de: &DirEntry) -> io::Result<()> {
        let dep = de.path();
        if self.regex.is_match(dep.to_str().unwrap()) {
            return self.strategy.execute(de);
        } else if dep.is_dir() {
            for entry in read_dir(dep).unwrap() {
                let entry: DirEntry = entry?;
                self.look_for_files(&entry)?;
                // let path = entry.path();
                // return if path.is_dir() {
                //     self.look_for_files(de)
                // } else {
                //     self.strategy.execute(de)
                // }
            }
        }
        Ok(())
    }
    pub fn act(&self) -> io::Result<()> {
        for entry in read_dir(self.root_dir_path).unwrap() {
            let entry: DirEntry = entry?;
            self.look_for_files(&entry)?;
        }
        Ok(())
    }
}

// pub trait FileManager {
//     // fn move_entry(de: &DirEntry, destination: &Path) -> io::Result<()>;
//
//     fn copy_entry(&self, de: &DirEntry, destination: &Path) -> io::Result<()>;
//
//     // fn remove_entry(de: &DirEntry) -> io::Result<()>;
// }
//
// pub trait Crypt {
//     fn encrypt(de: &DirEntry, key: &str, in_place: bool) -> io::Result<()>;
//
//     fn decrypt(de: &DirEntry, key: &str, in_place: bool) -> io::Result<()>;
// }
//
// pub trait Compressor {
//     fn compress(de: &DirEntry, in_place: bool) -> io::Result<()>;
//
//     fn decompress(de: &DirEntry, in_place: bool) -> io::Result<()>;
// }