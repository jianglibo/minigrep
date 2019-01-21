use std::fs::File;
use std::io::BufReader;
use std::string::String;
use std::io::Read;

// https://m4rw3r.github.io/rust-questionmark-operator
fn read_a_file(file_name: &str)  -> std::io::Result<()> {
    let file = File::open(file_name)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    assert_eq!(contents, "Hello, world!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::ffi;
    use std::path::{Path};
    #[test]
    fn test_read_a_file() {
        let result = super::read_a_file(&"afile");
        assert!(result.is_err());

        let path_result = env::current_dir();

        assert!(path_result.is_ok());
        let path = path_result.unwrap();

        let dir_name = path.file_name();
        let t_str = Some(ffi::OsStr::new("minigrep"));
        assert_eq!(t_str, dir_name);

        let new_path = path.join(Path::new("fixtrues/directory_access/directory_access_gb18030.txt")).canonicalize();

        let new_path = new_path.unwrap();
        // assert_eq!(new_path.to_str().unwrap(), "abc");
        let result = super::read_a_file(new_path.to_str().unwrap());
        match result {
            Ok(_) => (),
            Err(e) => println!("{}", e)
        }
        // assert!(result.is_ok());

    //     match super::read_a_file(&"foo.txt") {
    //         Ok(_) => (),
    //         Err(err) => println!("Error: {:?}", err)
    //    }
    }
}