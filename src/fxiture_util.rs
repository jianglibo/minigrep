use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use std::string::String;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::vec::Vec;

pub fn get_fixture_file(postfix: &[&str]) -> std::io::Result<PathBuf> {
    let mut path_result = env::current_dir()?;
    path_result = path_result.join("fixtrues");
    for x in postfix {
        // x is &&str
        path_result = path_result.join(x);
    }
    let new_path = path_result.canonicalize()?;
    Ok(new_path)
}

pub fn print_stars<T: AsRef<str>>(v: T) {
    println!("xxxxxxxxxxxxxx{}xxxxxxxxxxxx", v.as_ref());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_fixture_file() {
        let f = super::get_fixture_file(&["mysql", "my.cnf"]).unwrap();
        assert!(f.exists());
        let metadata = std::fs::metadata(f).unwrap();
        assert_eq!(metadata.len(), 987);
    }
}