use std::{fs, io};
use std::fs::DirEntry;
use crate::manager::{CopyStrategy, ManagementStrategy};


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
