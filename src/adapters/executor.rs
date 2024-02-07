use std::{fs, io};
use std::fs::DirEntry;
use std::path::Path;
use crate::manager::ManagementStrategy;


impl ManagementStrategy for CopyStrategy<'_> {
    fn execute(&self, de: &DirEntry) -> io::Result<()> {
        let destination_filename = self.destination.join(de.path().file_name().unwrap()) ;
        let copy_res = fs::copy(de.path(), destination_filename);
        match copy_res {
            Ok(code) => {
                let path = de.path();
                let from = path.to_str().unwrap();
                println!("Copied {from}, {code}");
                Ok(())
            }
            Err(err) => {
                Err(err)
            }
        }
    }
}

impl ManagementStrategy for ViewStrategy {
    fn execute(&self, de: &DirEntry) -> io::Result<()> {
        println!("{}", de.path().to_str().unwrap());
        Ok(())
    }
}

impl ManagementStrategy for DeleteStrategy {
    fn execute(&self, de: &DirEntry) -> io::Result<()> {
        return if de.path().is_dir() {
            fs::remove_dir(de.path())
        } else {
            fs::remove_file(de.path())
        }
    }
}

pub struct CopyStrategy<'a> {
    pub destination: &'a Path
}

pub struct ViewStrategy;

pub struct  DeleteStrategy;