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
pub struct MycnfBlock {
    name: String,
    lines: Vec<String>,
}

#[derive(Debug)]
pub struct MyCnfFile {
    pre_lines: Vec<String>,
    post_lines: Vec<String>,
    blocks: Vec<MycnfBlock>,
}

impl MyCnfFile {
    fn get_config(key: &str) -> Option<(&str, &str)> {
        Some(("a", "b"))
    }
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

pub fn get_mycnf<T: AsRef<Path>>(file: T) -> std::io::Result<MyCnfFile> {
    let file1 = File::open(file)?;
    let lines_result: std::result::Result<Vec<String>, _> = BufReader::new(file1).lines().collect();
    let lines = lines_result.unwrap();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\[(?P<blockname>[^\[\]]+)\]$").unwrap();
    }
    let mut mycnf = MyCnfFile {
        pre_lines: vec![],
        post_lines: vec![],
        blocks: vec![],
    };
    let mut block_lines: Vec<String> = Vec::new();
    let mut cur_block_name: Option<String> = None;

    for line in lines {
        let trimed_line: &str = line.trim();
        let caps_op = RE.captures(trimed_line);
        match caps_op {
            Some(caps) => {
                // is block name line, a new block name line.
                let bn = &caps["blockname"];
                if let Some(bn) = cur_block_name {
                    mycnf.blocks.push(MycnfBlock {
                        name: bn,
                        lines: block_lines,
                    });
                    block_lines = Vec::new();
                }
                cur_block_name = Some(bn.to_owned());
            }
            None => {
                // is block content line.
                if cur_block_name.is_some() {
                    block_lines.push(trimed_line.to_owned());
                } else {
                    mycnf.pre_lines.push(trimed_line.to_owned());
                }
            }
        }
    }
    if let Some(bn) = cur_block_name {
        if let Some(blk) = mycnf.blocks.last() {
            if blk.name != bn {
                mycnf.blocks.push(MycnfBlock {
                    name: bn,
                    lines: block_lines,
                });
            }
        } else {
            mycnf.blocks.push(MycnfBlock {
                name: bn,
                lines: block_lines,
            });
        }
    }
    Ok(mycnf)
}

#[cfg(test)]
mod tests {
    use crate::fxiture_util::{get_fixture_file, print_stars};
    use regex::Regex;

    #[test]
    fn test_get_lines<'a>() {
        // let lines = super::get_lines(get_fixture_file(&["mysql", "my.cnf"]).unwrap()).unwrap();
        // assert_eq!(lines.len(), 27);
        // assert_eq!(
        //     lines[0],
        //     "# For advice on how to change settings please see"
        // );
        let p = get_fixture_file(&["mysql", "my.cnf"]);
        let mycnf = super::get_mycnf(p.unwrap()).unwrap();

        let v = vec![1, 2, 3];
        if let Some(v1) = v.last() {
            assert_eq!(v1, &3);
        }

        assert_eq!(v.len(), 3);
        assert_eq!(mycnf.pre_lines.len(), 3);

        assert_eq!(mycnf.blocks.len(), 1);
    }
}
