#[cfg(test)]
mod tests {

    #[test]
    fn test_push_vec() {
        let vec1: Vec<u8> = Vec::with_capacity(5);
        assert_eq!(vec1.len(), 0);
        assert_eq!(vec1.capacity(), 5);

        let mut prev_u8: Vec<u8> = vec![0; 2];
        assert_eq!(prev_u8, [0, 0]);
        prev_u8.push(3);
        prev_u8.push(3);
        assert_eq!(prev_u8, [0, 0, 3, 3]);

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
        // assert_eq!(b'我', 66);
        let u8s = parse_hex("10");
        assert_eq!(u8s.get(0).unwrap(), &16);
        let s = r"\xce\xde\xb7\xa8\xb4\xf2\xbf\xaa\xce\xc4\xbc\xfe\xa1\xb0";
        let mut array: [u8; 2] = [0; 2];
        let mut prev_u8: Vec<u8> = vec![0; 2];
        s.as_bytes().iter().for_each(|it| {
            if prev_u8 == [b'\\', b'x'] {

            }
        });
    }

}