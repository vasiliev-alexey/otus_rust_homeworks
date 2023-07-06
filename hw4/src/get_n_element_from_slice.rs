pub fn get_element_from_slice<T>(values: &mut [T], n: usize) -> &mut T {
    if n >= values.len() || n == 0 {
        panic!("Index out of bounds");
    }
    &mut values[n]
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Mock {
        test: usize,
    }
    #[test]
    fn test_get_element() {
        let mut vec = vec![Mock { test: 0 }, Mock { test: 1 }];
        let res = get_element_from_slice(&mut vec[0..2], 1);
        assert_eq!(res.test, 1);
        let res = get_element_from_slice(&mut vec[0..2], 0);
        assert_eq!(res.test, 0);
    }
    #[test]
    #[should_panic]
    fn test_get_element_bigger_index() {
        let mut vec = vec![Mock { test: 0 }, Mock { test: 1 }];
        get_element_from_slice(&mut vec[0..2], 10);
    }
}
