#![feature(question_mark)]

#[macro_use] extern crate clap;

mod options;

use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, Read, Write};   
use std::path::{Path, PathBuf};
use options::Options;

fn main() {
    let options = options::read();
    
    match File::open(options.path()) {
        Err(e) => report_error("Could not open file", e),
        Ok(file) => remove_whitespace(file, &options),
    }
}

fn remove_whitespace<R: Read>(file: R, options: &Options) {
    match create_clean_file(options.path(), file) {
        Err(e) => report_error(&format!("Could not clean file {:?}", options.path()), e),
        Ok(clean_path) => if options.overwrite() {
            if let Err(e) = fs::rename(clean_path, options.path()) {
                report_error("unable to rename file", e);
            }
        }
    }
}

fn create_clean_file<R: Read>(path: &Path, file: R) -> Result<PathBuf, io::Error> {
    let path = {
        use std::ffi::{OsStr, OsString};
        
        let mut buf = path.to_path_buf();
        let mut extension = OsString::new();
        extension.push(buf.extension().expect("unable to get extension"));
        extension.push(OsStr::new(".clean"));
        
        buf.set_extension(extension);
        buf
    };
    
    let mut clean_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&path)?;
    
    for line in BufReader::new(file).lines() {
        writeln!(clean_file, "{}", line?.trim_right_matches(
            |c: char| c.is_whitespace() || c == '\n')
        )?;
    }
    
    Ok(path)
}

fn report_error<E: Error>(message: &str, error: E) {
    println!("{}: {}", message, error);
    std::process::exit(1);
}