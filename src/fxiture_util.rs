use std::env;
use std::path::{PathBuf};

pub fn get_fixture_file(postfix: &[&str], canonicalize: bool) -> std::io::Result<PathBuf> {
    let mut path_result = env::current_dir()?;
    path_result = path_result.join("fixtrues");
    for x in postfix {
        // x is &&str
        path_result = path_result.join(x);
    }
    if canonicalize {
        Ok(path_result.canonicalize()?)
    } else {
        Ok(path_result)
    }
}

pub fn print_stars<T: AsRef<str>>(v: T) {
    println!("xxxxxxxxxxxxxx{}xxxxxxxxxxxx", v.as_ref());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_fixture_file() {
        let f = super::get_fixture_file(&["mysql", "my.cnf"], true).unwrap();
        assert!(f.exists());
        let metadata = std::fs::metadata(f).unwrap();
        assert_eq!(metadata.len(), 987);
    }
}