#[cfg(test)]
mod tests {
    use std::vec::Vec;
    #[test]
    fn test_index() {
        let ms = "abc";
        let chars: Vec<char> = ms.chars().collect();
        assert_eq!('a', chars[0]);

        let ms = "しばらく";
        let chars: Vec<char> = ms.chars().collect();
        assert_eq!('し', chars[0]);

        let ms = " [mysqld] ";
        let ms = ms.trim();

        let mut v: Vec<_> = ms.match_indices(|c| {c == '[' || c == ']'}).collect();
        assert_eq!(v, [(0, "["), (7, "]")]);
        let start = v.remove(0);
        assert_eq!(start, (0, "["));
        let end = v.pop();
        assert_eq!(end, Some((7, "]")));
        assert_eq!(end.unwrap().0, 7);

        let t = (0, 1, 2);
        assert_eq!(t.2, 2);
        let bn = &ms[1..end.unwrap().0];
        assert_eq!(bn, "mysqld");
    }
    
}