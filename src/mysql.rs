use std::convert::AsRef;
use std::env;
use std::ffi;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Read;
use std::vec::Vec;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use std::string::String;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::error::Error;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
struct MycnfBlock {
    name: String,
    lines: Vec<String>,
}

#[derive(Debug)]
struct MyCnfFile {
    pre_lines: Vec<String>,
    post_lines: Vec<String>,
    blocks: Vec<MycnfBlock>,
}

// fn get_lines<'a, T: AsRef<Path>>(file: T) -> std::io::Result<&'a [&'a str]> {
// fn get_lines<'a, T: AsRef<Path>>(file: T) -> std::io::Result<std::io::Lines<String>> {
fn get_lines<'a, T: AsRef<Path>>(file: T) -> std::io::Result<Vec<String>> {
    let file = File::open(file)?;
    BufReader::new(file).lines().collect()
    // let lines: std::io::Result<Vec<String>> = bf.lines().collect();
    // let lines: Vec<std::io::Result<String>> = bf.lines().collect();
    // lines
   }

#[cfg(test)]
mod tests {
    use crate::fxiture_util::get_fixture_file;
    use regex::Regex;
    #[test]
    fn test_get_lines() {
        let lines = super::get_lines(get_fixture_file(&["mysql", "my.cnf"]).unwrap()).unwrap();
        assert_eq!(lines.len(), 27);
        assert_eq!(lines[0], "# For advice on how to change settings please see");
        let mycnf = super::MyCnfFile {
            pre_lines: vec![],
            post_lines: vec![],
            blocks: vec![],
        };
        lazy_static! {
           static ref RE: Regex = Regex::new(r"^\[(?P<blocknREe>.*)\]$").unwrap();
        }
        let mut cur_block_name: Option<&str> = None;
        let mut cur_block: Option<super::MycnfBlock> = None;
        for line in lines {
            let trimed_line: &str = line.trim();
            let cap = RE.captures(trimed_line);
            match cap {
                Some(cap) => {
                    let bn = &cap["blockname"];
                },
                None => ()
            }
        }
    }
}