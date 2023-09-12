pub fn split_slice_by2<T>(values: &[T], n: usize) -> [&[T]; 2] {
    if n >= values.len() || n == 0 {
        panic!("Index out of bounds");
    };
    let (left, right) = values.split_at(n);
    [left, right]
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Mock {
        test: usize,
    }
    #[test]
    fn test_get_element() {
        let mut vec = vec![
            Mock { test: 0 },
            Mock { test: 1 },
            Mock { test: 2 },
            Mock { test: 3 },
        ];
        let vec = split_slice_by2(&mut vec[0..4], 1);
        assert_eq!(vec.get(0).unwrap().len(), 1);
        assert_eq!(vec.get(0).unwrap().get(0).unwrap().test, 0);
        assert_eq!(vec.get(1).unwrap().len(), 3);
        assert_eq!(vec.get(1).unwrap().get(2).unwrap().test, 3);
    }
    #[test]
    #[should_panic]
    fn test_get_element_bigger_index() {
        let mut vec = vec![Mock { test: 0 }, Mock { test: 1 }];
        split_slice_by2(&mut vec[0..2], 10);
    }
}
