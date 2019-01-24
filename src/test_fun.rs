#[cfg(test)]
mod tests {

    // I got a reference with lifetime.
    fn return_ref<'a>() -> &'a str {
        let s = "abc";
        return s;
    }
    // Need lifetime.
    fn return_a_vec<'a>() -> Vec<&'a str> {
        vec!["a", "b"]
    }

    fn return_a_vec1() -> Vec<String> {
        vec![String::from("a")]
    }

    //E0515
    // fn return_a_vec2<'a>() -> &'a Vec<String> {
    //     let mut v = Vec::new();
    //     v.push(String::from("a"));
    //     &v
    // }

    // Box doesn't help.
    // fn return_box_vec<'a>() -> Box<&'a Vec<String>> {
    //     let mut v = Vec::new();
    //     v.push(String::from("a"));
    //     Box::new(&v)
    // }

    #[test]
    fn test_1() {
        assert_eq!(return_ref(), "abc");
        assert_eq!(return_a_vec(), ["a", "b"]);
        assert_eq!(return_a_vec1()[0], "a".to_owned());
    }
}