use std::env;
use std::path::{Path, PathBuf};
use clap::ArgMatches;

pub struct Options {
    path: PathBuf,
    overwrite: bool,
}

impl Options {
    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }
    
    pub fn overwrite(&self) -> bool {
        self.overwrite
    }
}

pub fn read() -> Options {
    let matches = get_matches();
    Options {
        path: create_path(&matches.value_of("path").expect("path not provided")), 
        overwrite: matches.is_present("overwrite")
    }
}

fn create_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir().expect("unable to get current directory").join(path)
    }
}

fn get_matches<'a>() -> ArgMatches<'a> {
    clap_app!(unwhiten =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "A whitespace scrubber")
        (@arg path: +required "File path")
        (@arg overwrite: -o --overwrite "Overwrite original file")
    ).get_matches()
}