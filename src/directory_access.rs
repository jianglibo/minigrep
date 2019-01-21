



#[cfg(test)]
mod tests {
    use std::env;
    use std::ffi;
    use std::path::{Path, PathBuf};
    use std::string::String;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::Read;

    // https://m4rw3r.github.io/rust-questionmark-operator
    fn read_a_file(file_name: &Path)  -> std::io::Result<()> {
        let file = File::open(file_name)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        assert_eq!(contents, "Hello, world!");
        Ok(())
    }

    fn get_fixture_file(postfix: &str) -> std::io::Result<PathBuf> {
        let path_result = env::current_dir()?;
        let t_file_prefix: &str = "fixtrues/directory_access/directory_access_";
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
            Err(e) => println!("xxxxxxxxxxxxxx{}xxxxxxxxxxxxxxxxxxxxxxxx", e)
        }
    }

    #[test]
    fn test_read_a_file() {
        // let result = read_a_file("afile");
        // assert!(result.is_err());
        let path_result = env::current_dir();
        assert!(path_result.is_ok());
        let path = path_result.unwrap();

        let dir_name = path.file_name();
        let t_str = Some(ffi::OsStr::new("minigrep"));
        assert_eq!(t_str, dir_name);
        let t_file_prefix: &str = "fixtrues/directory_access/directory_access_";
        let t_file_prefix_1 = String::from("fixtrues/directory_access/directory_access_");

        assert_eq!(t_file_prefix, t_file_prefix_1);
        // str slice is valid utf8.
        let t_file = format!("{}{}", t_file_prefix, "gb18030.txt");
        assert_eq!(t_file, "fixtrues/directory_access/directory_access_gb18030.txt");
        assert!(t_file_prefix.len() > 0);
    }
}