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
     // Vec is an owned value. String too.
    pre_lines: Vec<String>,
    blocks: Vec<MycnfBlock>,
}

impl MyCnfFile {
    // Because Option implements the Copy trait.
    fn get_config(&self, block_name: &str, key_name: &str) -> Option<(String, String)> {
        let b_op = &self.blocks.iter().find(|blk| blk.name == block_name);
        if let Some(b) = b_op {
            let kv: Option<(String, String)> = b.lines.iter().find_map(|line| {
                if line.starts_with('#') {
                    return None;
                } else {
                    let pair: Vec<&str> = line.splitn(2, '=').collect();
                    if pair.len() == 2 && pair[0].trim() == key_name {
                        return Some((pair[0].trim().to_owned(), pair[1].trim().to_owned()));
                    } else {
                        return None;
                    }
                }
            });
            return kv;
        }
        None
    }

    fn set_block_key_value(&mut self, block_name: &str, key_name: &str, value: &str) {
        let b_op = &self.blocks.iter().find(|blk| blk.name == block_name);
        if let Some(b) = b_op {
            println!("{:?}",b);
        } else {
            &self.blocks.push(MycnfBlock {
                name: block_name.to_owned(),
                lines: vec![format!("{}={}", key_name, value)]
            });
        }
    }
}

fn get_lines<'a, T: AsRef<Path>>(file: T) -> std::io::Result<Vec<String>> {
    let file = File::open(file)?;
    BufReader::new(file).lines().collect()
}

pub fn get_mycnf<T: AsRef<Path>>(file: T) -> std::io::Result<MyCnfFile> {
// pub fn get_mycnf<'a>(lines: &'a Vec<String>) -> std::io::Result<MyCnfFile<'a>> {
    let file1 = File::open(file)?;
    let lines_result: std::result::Result<Vec<String>, _> = BufReader::new(file1).lines().collect();
    let lines = lines_result.unwrap();

    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\[(?P<blockname>[^\[\]]+)\]$").unwrap();
    }
    let mut mycnf = MyCnfFile {
        pre_lines: vec![],
        blocks: vec![],
    };
    let mut block_lines: Vec<String> = Vec::new();
    let mut cur_block_name: Option<String> = None;

    for line in lines {
        let trimed_line = line.trim();
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
        // let lines = super::get_lines(p.unwrap()).unwrap();
        let mycnf = super::get_mycnf(p.unwrap()).unwrap();

        let v = vec![1, 2, 3];
        if let Some(v1) = v.last() {
            assert_eq!(v1, &3);
        }

        assert_eq!(v.len(), 3);
        assert_eq!(mycnf.pre_lines.len(), 3);

        assert_eq!(mycnf.blocks.len(), 1);

        let trim_str = |s: &'a str| -> &'a str { 
            s.trim()
        };

        let s_trimed = trim_str(" abc ");
        assert_eq!(s_trimed, "abc");

        let kv = mycnf.get_config("mysqld", "log-error");
        let expect = ("log-error".to_owned(), "/var/log/mysqld.log".to_owned());

        assert_eq!(kv.unwrap(), expect);
    }

    #[test]
    fn test_trim() {
        let s = "#  abc=55 ";
        let s_trimed = s.trim_start_matches('#').trim();
        let ss: Vec<&str> = s_trimed.splitn(2, '=').collect();
        assert_eq!(ss[0], "abc");
        assert_eq!(ss[1], "55");

        let s = " [mysqld] ";
        assert!("][".contains(']'));
        assert!("][".contains('['));
        let v: Vec<_> = s.trim().match_indices(|c| "[]".contains(c)).collect();
        let s_trimed = s.trim();
        assert!(s_trimed.starts_with('['));
        let content = &(s_trimed[1..s_trimed.len()-1]);
        assert_eq!(content, "mysqld");

        let x: &[_] = &['[', ']'];
        let ss = s_trimed.trim_matches(x);
        assert_eq!(ss, "mysqld");
        assert_eq!(v, [(0, "["), (7, "]")]);
    }
}
