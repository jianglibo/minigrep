use regex::Regex;
use std::convert::AsRef;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::string::String;
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
    fn write_to_file<T: AsRef<Path>>(self, file: T) -> std::io::Result<()> {
        let mut file_w = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(file)?;
        self.pre_lines.iter().for_each(|line| {
            writeln!(file_w, "{}", line).unwrap();
        });

        self.blocks.iter().for_each(|b| {
            writeln!(file_w, "[{}]", b.name).unwrap();
            b.lines.iter().for_each(|line| {
                writeln!(file_w, "{}", line).unwrap();
            });
        });
        Ok(())
    }

    fn from_file<T: AsRef<Path>>(file: T) -> std::io::Result<MyCnfFile> {
        let file1 = File::open(file)?;
        let lines_result: std::result::Result<Vec<String>, _> =
            BufReader::new(file1).lines().collect();
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

    fn remove_block_key_value(self: &mut Self, block_name: &str, key_name: &str) {
        let b_op = self.blocks.iter_mut().find(|blk| blk.name == block_name);
        if let Some(b) = b_op {
            for line in b.lines.iter_mut() {
                let pair: Vec<_> = line.trim().splitn(2, '=').collect();
                if pair.len() == 2 && pair[0].trim() == key_name {
                    *line = format!("#{}", line);
                }
            }
        }
    }

    // mut self, this is start point. you can change self.
    // self.blocks.iter_mut to borrow mutable block.
    // mutability was determined at borrow point.
    fn set_block_key_value(self: &mut Self, block_name: &str, key_name: &str, value: &str) {
        let b_op = self.blocks.iter_mut().find(|blk| blk.name == block_name);
        if let Some(b) = b_op {
            // find take a reference, So the closure parameter is double referenced.
            // let r = b.lines.iter_mut().find(|line| {
            //     let pair: Vec<_> = line.trim_start_matches('#').trim().splitn(2, '=').collect();
            //     if pair.len() == 2 && pair[0].trim() == key_name {
            //         *line = format!("{}={}", key_name, value);
            //         true
            //     } else {
            //         false
            //     }
            // });
            let mut done = false;

            // if key value already exists.
            for line in b.lines.iter_mut() {
                let pair: Vec<_> = line.trim().splitn(2, '=').collect();
                if pair.len() == 2 && pair[0].trim() == key_name {
                    if done {
                        // duplicated key value line. must comment out.
                        *line = String::from("");
                    } else {
                        *line = format!("{}={}", key_name, value);
                        done = true;
                    }
                }
            }

            if !done {
                for line in b.lines.iter_mut() {
                    let pair: Vec<_> = line.trim_start_matches('#').trim().splitn(2, '=').collect();
                    if pair.len() == 2 && pair[0].trim() == key_name {
                        *line = format!("{}={}", key_name, value);
                        done = true;
                        break;
                    }
                }
            }
            if !done {
                b.lines.push(format!("{}={}", key_name, value));
            }
        } else {
            &self.blocks.push(MycnfBlock {
                name: block_name.to_owned(),
                lines: vec![format!("{}={}", key_name, value)],
            });
        }
    }
}

fn get_lines<'a, T: AsRef<Path>>(file: T) -> std::io::Result<Vec<String>> {
    let file = File::open(file)?;
    BufReader::new(file).lines().collect()
}

#[cfg(test)]
mod tests {
    use crate::fxiture_util::{get_fixture_file, print_stars};
    use regex::Regex;
    use yaml_rust::{YamlLoader, YamlEmitter};
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_get_lines<'a>() {
        // let lines = super::get_lines(get_fixture_file(&["mysql", "my.cnf"]).unwrap()).unwrap();
        // assert_eq!(lines.len(), 27);
        // assert_eq!(
        //     lines[0],
        //     "# For advice on how to change settings please see"
        // );
        // let [a, c: Vec<_>, b] = [1, 2, 3, 4];
        let p = get_fixture_file(&["mysql", "my.cnf"], true);

        // let lines = super::get_lines(p.unwrap()).unwrap();
        let mut mycnf = super::MyCnfFile::from_file(p.unwrap()).unwrap();

        let v = vec![1, 2, 3];
        if let Some(v1) = v.last() {
            assert_eq!(v1, &3);
        }

        assert_eq!(v.len(), 3);
        assert_eq!(mycnf.pre_lines.len(), 3);

        assert_eq!(mycnf.blocks.len(), 1);

        let trim_str = |s: &'a str| -> &'a str { s.trim() };

        let s_trimed = trim_str(" abc ");
        assert_eq!(s_trimed, "abc");

        let kv = mycnf.get_config("mysqld", "log-error1");
        assert!(kv.is_none());

        let kv = mycnf.get_config("mysqld", "log-error");
        let expect = ("log-error".to_owned(), "/var/log/mysqld.log".to_owned());

        mycnf.set_block_key_value("mysqld", "a", "b");
        let kv = mycnf.get_config("mysqld", "a");
        let expect = ("a".to_owned(), "b".to_owned());
        assert_eq!(kv.unwrap(), expect);

        mycnf.remove_block_key_value("mysqld", "a");
        let kv = mycnf.get_config("mysqld", "a");
        assert_eq!(kv, None);

        let p_out = get_fixture_file(&["notingit", "my.cnf"], false).unwrap();
        println!("{:?}", p_out);
        println!("---------------------{}-----------------", 1);
        mycnf.write_to_file(p_out).unwrap();
    }

    #[test]
    fn test_trim() {
        let f = false;
        if !f {
            println!("{}", "yes");
        }
        let mut counter = 0;
        let result = for i in (1..3) {};
        assert_eq!((), result);
        let result = 'abreadkablefor: for i in (1..3) {
            break 'abreadkablefor;
        };
        assert_eq!((), result);
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        assert_eq!(result, 20);
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
        let content = &(s_trimed[1..s_trimed.len() - 1]);
        assert_eq!(content, "mysqld");

        let x: &[_] = &['[', ']'];
        let ss = s_trimed.trim_matches(x);
        assert_eq!(ss, "mysqld");
        assert_eq!(v, [(0, "["), (7, "]")]);
    }
    #[test]
    fn test_config() {
        let p = get_fixture_file(&["mysql", "mysql_configuration.1.yml"], true);

        let mut file1 = File::open(p.unwrap()).unwrap();

        let mut buf = String::new();

        file1.read_to_string(&mut buf).unwrap();
        
        let docs = YamlLoader::load_from_str(&buf).unwrap();

        // Multi document support, doc is a yaml::Yaml
        let doc = &docs[0];

        // Index access for map & array
        assert_eq!(doc["AppName"].as_str().unwrap(), "mysql");
        assert_eq!(doc["MysqlVersion"].as_str().unwrap(), "57");
        assert_eq!(doc["MysqlVersion"].as_str().unwrap().parse::<i32>().unwrap(), 57);
        assert_eq!(doc["taskcmd"].as_hash().unwrap().len(), 3);

        let node = &(doc["taskcmd"]);
        println!("{:#?}", node);

        // let idhash = doc["Software"]["InstallDetect"].as_hash().unwrap();
        // assert_eq!(idhash.get(&"unexpect").unwrap().as_str().unwrap(), "not-found");

        let node = &(doc["Software"]["InstallDetect"]);
        
        assert_eq!(node["command"].as_str().unwrap(), "systemctl status mysqld");
        // Chained key/array access is checked and won't panic,
        // return BadValue if they are not exist.
        assert!(doc["INVALID_KEY"][100].is_badvalue());
    }

}
