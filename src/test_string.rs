#[cfg(test)]
mod tests {
    use crate::fixture_util::print_stars;
    use std::vec::Vec;

    #[derive(Debug)]
    struct AStruct {
        name: String,
        lines: Vec<String>,
    }

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

        let mut v: Vec<_> = ms.match_indices(|c| c == '[' || c == ']').collect();
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

    #[test]
    fn test_match() {
        let s = (0, "a");
        let s1 = "a";

        match s.1 {
            _ if s.1 == s1 => print_stars(s1),
            _ => print_stars("not matches."),
        }
    }

    fn get_ass<T: AsRef<str>>(name: T) -> AStruct {
        let t = AStruct {
            name: String::from(name.as_ref()),
            lines: vec![String::from("hello")],
        };
        return t;
    }

    #[test]
    fn test_compare() {
        let s = "abc";
        let ss = String::from("abc");
        assert!(s == ss);
        assert_eq!(s, ss);

        let a_struct = AStruct {
            name: String::from("abc"),
            lines: vec!["hello".to_owned()],
        };

        let mut as1: Vec<Option<&AStruct>> = Vec::new();

        assert!(as1.len() == 0);

        for i in &[1, 2, 3] {
            println!("{}", i);
            as1.push(Some(&a_struct));
            println!("{:#?}", as1);
            assert!(as1.len() == *i);
        }
        // immutability is for variables.
        let mut _v_br = "a";
        _v_br = "b";
        assert_eq!(_v_br, "b");

        let one_as = a_struct;
        println!("{:?}", one_as);

        let mut as_br: Option<AStruct> = Some(get_ass("aaa"));

        as_br = Some(get_ass("bbb"));

        println!("------------{:?}-------------", as_br.unwrap());
    }

    #[test]
    fn test_bor() {
        let mut i_br: &i32 = &5;
        assert_eq!(*i_br, 5);

        i_br = &10;
        assert_eq!(*i_br, 10);

        let v = [1, 2, 3];

        let v1 = v;

        assert_eq!(v1, [1, 2, 3]);

        let mut v2 = &v;

        assert_eq!(v1, *v2);
        assert_eq!(&v1, v2);

        let v3 = &mut [1, 2, 3];

        for i in v3.iter_mut() {
            *i = 2;
        }

        let l = v3.len();

        println!("{:?}", l);

        v3[0] = 55;

        assert_eq!(*v3, [55, 2, 2]);
    }

    #[test]
    fn test_scope() {
        // This binding lives in the main function
        let mut long_lived_binding = 1;

        // This is a block, and has a smaller scope than the main function
        {
            // This binding only exists in this block
            let short_lived_binding = 2;

            println!("inner short: {}", short_lived_binding);

            // This binding *shadows* the outer one
            long_lived_binding = 5;

            println!("inner long: {}", long_lived_binding);
        }
        // End of the block

        // Error! `short_lived_binding` doesn't exist in this scope
        // println!("outer short: {}", short_lived_binding);
        // FIXME ^ Comment out this line

        println!("outer long: {}", long_lived_binding);

        // This binding also *shadows* the previous binding
        let long_lived_binding = 'a';

        println!("outer long: {}", long_lived_binding);
    }

    #[test]
    fn test_bool() {
        let y = true;
        assert!(y);
        match y {
            true => (),
            false => (),
        }
    }

    #[test]
    fn test_split_str() {
        let t = [1, 2];
        let [a, b] = [1, 2];
        assert!(a == 1);
        assert!(b == 2);

        let (a, b) = (1, 2);
        assert!(a == 1);
        assert!(b == 2);
        assert!(t.len() == 2);

        match t {
            [a, b] => println!("{}{}", a, b),
        };
        // let [a, b]: &str = "a=b=c".splitn(2, '=').into();
    }

    #[test]
    fn test_split_str1() {
        fn parse_hex(hex_asm: &str) -> Vec<u8> {
            let mut hex_bytes = hex_asm
                .as_bytes()
                .iter()
                .filter_map(|b| match b {
                    b'0'...b'9' => Some(b - b'0'),
                    b'a'...b'f' => Some(b - b'a' + 10),
                    b'A'...b'F' => Some(b - b'A' + 10),
                    _ => None,
                })
                .fuse();
            let mut bytes = Vec::new();
            while let (Some(h), Some(l)) = (hex_bytes.next(), hex_bytes.next()) {
                bytes.push(h << 4 | l)
            }
            bytes
        }
        // 1 => 00000001 << 4 => 00010000
        // 48-57 => 0 - 9,  65-90 => A-Z, 97-122 => a-z
        // Decimal	98_222
        // Hex	0xff
        // Octal	0o77
        // Binary	0b1111_0000
        // Byte (u8 only)	b'A'
        let i: u8 = 2;
        // https://en.wikipedia.org/wiki/Logical_shift
        assert_eq!(0b0000_0001 << 4, 16);
        // assert_eq!("0x10".parse::<u8>().unwrap(), 16);
        assert_eq!(b'0', 48);
        let u8s = parse_hex("10");
        assert_eq!(u8s.get(0).unwrap(), &16);
    }
}
