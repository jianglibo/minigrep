#[cfg(test)]
mod tests {
    use std::rc::Rc;

    #[test]
    fn test_scan() {
        let s = r"\xce\xde\xb7\xa8\xb4\xf2\xbf\xaa\xce\xc4\xbc\xfe\xa1\xb0";
        // let s = r"\xce\xde";
        let v_slice: &[u8] = s.as_bytes();

        // let () = b'0'...b'9' | b'a'...b'z' | b'A'...b'Z';
        
        let mut iter = v_slice.iter().scan((0u8, 0u8), |state, it| {
            match state {
                (b'\\', b'x') => {
                    state.0 = *it;
                    None
                },
                (b'0'...b'9', b'x') | (b'a'...b'f', b'x') | (b'A'...b'F', b'x') => {
                    match it {
                        b'0'...b'9' | b'a'...b'f' | b'A'...b'F' => Some(state.0, *it),
                        _ => {
                            
                        }
                    }
                    None
                },
                _ => None,
            }
        });
    }

    // You cannot use one iterator two times.
    #[test]
    fn test_first() {

        let s = r"\xce\xde\xb7\xa8\xb4\xf2\xbf\xaa\xce\xc4\xbc\xfe\xa1\xb0";
        // let s = r"\xce\xde";
        let v_slice: &[u8] = s.as_bytes();
        let mut first_item = None;
        if let Some((first, _)) = v_slice.split_first() {
            first_item = Some(first);
        }
        // print!("{}", s);
        assert_eq!(first_item, Some(&b'\\'));
        let bytes_enum = v_slice.iter().enumerate();
        // let mut bytes_it = Rc::new(v_slice.iter().peekable());
        let mut bytes_it = v_slice.iter().peekable();
        let bytes_it_clone = bytes_it.clone();
        // let std::iter::Map<std::iter::Peekable<std::slice::Iter<'_, u8>> = bytes_it.map(|x| x -1);
        let pair_it = bytes_it_clone.map(|&asc| match asc {
            b'\\' => {
                if let Some(&&c) = bytes_it.peek() {
                    if c == b'x' {
                        bytes_it.next();
                    }
                    None
                } else {
                    None
                }
            },
            b'0'...b'9' | b'a'...b'f' | b'A'...b'F' => {
                if let Some(&&c) = bytes_it.peek() {
                    match c {
                        b'0'...b'9' | b'a'...b'f' | b'A'...b'F' => {
                            bytes_it.next();
                            println!("..........{}..{}............", &asc, &c);
                            Some((asc, c))
                        },
                        _ => None
                    }
                } else {
                    None
                }
            },
            _ => None
        }).filter_map(|it| 
            it
        ).collect::<Vec<(u8, u8)>>();
        println!("{:?}", pair_it);
        assert_eq!(pair_it.len(), 0);
    }
}