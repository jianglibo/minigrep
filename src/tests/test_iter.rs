#[cfg(test)]
mod tests {
    use std::rc::Rc;

    #[test]
    fn test_batching() {
        assert_eq!(0, 0);
    }
    // scan stop on None.
    #[test]
    fn test_scan() {
        // let s = r"\xce\xde\xb7\xa8\xb4\xf2\xbf\xaa\xce\xc4\xbc\xfe\xa1\xb0";
        let vi: Vec<usize> = [1, 2, 3].iter().scan(1, |_, &it| {
            if it != 2 {
                Some(it)
            } else {
                None
            }
        }).collect();

        assert_eq!(vi.len(), 1);


        type ScanResult = Result<(u8, u8), ()>;

        // let mut iter = v_slice.iter().scan((None, None), |state: &mut (Option<u8>, Option<u8>), &it| {
        fn get_pair(a_slice: &[u8]) -> Vec<ScanResult> {
            a_slice.iter().scan((None, None), |state: &mut (Option<u8>, Option<u8>), &it| {
                match state {
                    (None, None) => {
                        state.0 = Some(it);
                        Some(Err(()))
                    },
                    (Some(_), None) => {
                        state.1 = Some(it);
                        Some(Err(()))
                    },
                    // \x
                    (Some(b'\\'), Some(b'x')) => {
                        state.0 = Some(it);
                        Some(Err(()))
                    },
                    // cx
                    (Some(b'0'...b'9'), Some(b'x')) | (Some(b'a'...b'f'), Some(b'x')) | (Some(b'A'...b'F'), Some(b'x')) => match it {
                        b'0'...b'9' | b'a'...b'f' | b'A'...b'F' => {
                            let tp = (state.0.unwrap(), it);
                            *state = (None, None);
                            Some(Ok(tp))
                        },
                        _ => {
                            // it value is leaked. cx -> it = g? discard all.
                            *state = (None, None);
                            Some(Err(()))
                        },
                    },
                    // we got 2 items, but first is leaked, for example it is 'ga', replace state with new pair.
                    _ => {
                        *state = (Some(it), None);
                        Some(Err(()))
                    },
               }
            }).filter(Result::is_ok).collect()
        }

        let s = r"\xce\xde";
        let v_slice = s.as_bytes();
        assert_eq!(get_pair(v_slice).len(), 2);

        let s = r"ce\xce\xde";
        let v_slice = s.as_bytes();
        assert_eq!(get_pair(v_slice).len(), 2);

        let s = r"ce\xce\xde1234";
        let v_slice = s.as_bytes();
        assert_eq!(get_pair(v_slice).len(), 2);

        // this will happen.
        let s = r"cxe\xce\xde";
        let v_slice = s.as_bytes();
        assert_eq!(get_pair(v_slice).len(), 2);
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