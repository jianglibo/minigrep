#[cfg(test)]
mod tests {
    use std::env;
    use std::ffi;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::Read;
    use std::path::{Path, PathBuf};
    use std::string::String;
    use std::time::{Duration, Instant};
    use std::thread::sleep;
    // use std::vec::Vec;
    use std::iter::FromIterator;
    use std::convert::AsRef;

    // https://m4rw3r.github.io/rust-questionmark-operator
    fn read_a_file<T: AsRef<Path>>(file_name: T) -> std::io::Result<String> {
        let file = File::open(file_name)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn get_fixture_file(postfix: &str) -> std::io::Result<PathBuf> {
        let path_result = env::current_dir()?;
        let t_file_prefix: &str = "fixtures/directory_access/directory_access_";
        let t_file = format!("{}{}{}", t_file_prefix, postfix, ".txt");
        let new_path = path_result.join(Path::new(&t_file)).canonicalize()?;
        Ok(new_path)
    }

    #[test]
    fn test_read_gb18030() {
        let t_file = get_fixture_file("gb18030").unwrap();
        let result = read_a_file(&t_file);

        match result {
            Ok(_) => (),
            Err(e) => println!("xxxxxxxxxxxxxx{}xxxxxxxxxxxxxxxxxxxxxxxx", e),
        }
    }

    #[test]
    fn test_read_utf8() {
        let t_file = get_fixture_file("utf8").unwrap();
        let result = read_a_file(&t_file);
        let contents = result.unwrap();
        let c = "开始学习RUST。";
        assert_eq!(contents, c);
        // There is no more direct way because char is a 32-bit Unicode scalar value, 
        // and strings in Rust are sequences of bytes (u8) representing text in UTF-8 encoding. They do not map directly to sequences of chars.
        let t_file = get_fixture_file("utf8_bom").unwrap();
        let result = read_a_file(&t_file);
        let contents = result.unwrap();
        let utf8_bom = '\u{feff}';

        let mut chars = contents.chars();
        let px = chars.next();
        assert_eq!(px, Some(utf8_bom));

        let c = "开始学习RUST。";
        // chars.fold(init: B, mut f: F)
        // let mut v: Vec<char> = c.chars().collect();
        // v.remove(0);
        let c1_bom_chopped = String::from_iter(chars);
        assert_eq!(c, c1_bom_chopped);
    }

    #[test]
    fn test_read_a_file() {
        // let result = read_a_file("a_file");
        // assert!(result.is_err());
        let path_result = env::current_dir();
        assert!(path_result.is_ok());
        let path = path_result.unwrap();

        let dir_name = path.file_name();
        let t_str = Some(ffi::OsStr::new("minigrep"));
        assert_eq!(t_str, dir_name);
        let t_file_prefix: &str = "fixtures/directory_access/directory_access_";
        let t_file_prefix_1 = String::from("fixtures/directory_access/directory_access_");

        assert_eq!(t_file_prefix, t_file_prefix_1);
        // str slice is valid utf8.
        let t_file = format!("{}{}", t_file_prefix, "gb18030.txt");
        assert_eq!(
            t_file,
            "fixtures/directory_access/directory_access_gb18030.txt"
        );
        assert!(t_file_prefix.len() > 0);
    }

    #[test]
    fn test_std_time() {
        let now = Instant::now();
        // we sleep for 2 seconds
        sleep(Duration::new(2, 0));
        // it prints '2'
        println!("xxxxxxx{}xxxxxxxxxx", now.elapsed().as_secs());
    }

}
