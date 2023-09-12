pub fn get_element_from_end<T>(values: &mut [T], n: usize) -> &T {
    if n >= values.len() {
        panic!("Index out of bounds");
    }
    &values[values.len() - 1 - n]
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Mock {
        test: usize,
    }
    #[test]
    fn test_get_element_from_end() {
        let mut vec = vec![Mock { test: 0 }, Mock { test: 1 }];
        let res = get_element_from_end(&mut vec[0..2], 1);
        assert_eq!(res.test, 0);
        let res = get_element_from_end(&mut vec[0..2], 0);
        assert_eq!(res.test, 1);
    }
    #[test]
    #[should_panic]
    fn test_get_element_bigger_index() {
        let mut vec = vec![Mock { test: 0 }, Mock { test: 1 }];
        get_element_from_end(&mut vec[0..2], 10);
    }
}
