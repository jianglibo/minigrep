use regex::Regex;
use std::collections::HashMap;
use std::convert::AsRef;
use std::env;
use std::error::Error;
use std::ffi;
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

// It's better to hold owned value for a struct.
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
    fn test_get_lines<'a>() {
        let lines = super::get_lines(get_fixture_file(&["mysql", "my.cnf"]).unwrap()).unwrap();
        assert_eq!(lines.len(), 27);
        assert_eq!(
            lines[0],
            "# For advice on how to change settings please see"
        );
        let mut mycnf = super::MyCnfFile {
            pre_lines: vec![],
            post_lines: vec![],
            blocks: vec![],
        };
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\[(?P<blockname>[^\[\]]+)\]$").unwrap();
        }

        let mut cur_block: Option<super::MycnfBlock> = None;

        let new_block = |bn: &str| {
            Some(super::MycnfBlock {
                name: String::from(bn),
                lines: vec![],
            })
        };

        let mut block_lines: Vec<String> = Vec::new();

        for line in lines {
            let trimed_line: &str = line.trim();
            let caps_op = RE.captures(trimed_line);
            match caps_op {
                Some(caps) => {
                    // is block name line.
                    let bn = &caps["blockname"];
                    if cur_block.is_some() {
                        let cb = cur_block.unwrap();
                        for bl in block_lines {
                            cb.lines.push(bl);
                        }
                        mycnf.blocks.push(cb);
                    }
                    cur_block = new_block(bn);
                }
                None => {
                    // is block content line.
                    if cur_block.is_some() {
                        block_lines.push(trimed_line.to_owned());
                        // cur_block.unwrap().lines.push(trimed_line.to_owned());
                    } else {
                        mycnf.pre_lines.push(trimed_line.to_owned());
                    }
                }
            }
        }
    }
}
