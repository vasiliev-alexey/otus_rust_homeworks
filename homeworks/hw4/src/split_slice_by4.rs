pub fn split_slice_by4<T>(values: &[T]) -> [&[T]; 4] {
    let (left, right) = values.split_at(values.len() / 2);
    let (one, two) = left.split_at(left.len() / 2);
    let (three, four) = right.split_at(right.len() / 2);
    [one, two, three, four]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Debug)]
    struct Mock {
        test: usize,
    }
    #[test]
    fn test_get_element_for_4() {
        let mut vec = vec![
            Mock { test: 0 },
            Mock { test: 1 },
            Mock { test: 2 },
            Mock { test: 3 },
        ];
        let vec = split_slice_by4(&mut vec[0..4]);
        assert_eq!(vec[0].len(), 1);
        assert_eq!(vec[0].get(0).unwrap().test, 0);
        assert_eq!(vec[1].len(), 1);
        assert_eq!(vec[2].len(), 1);
        assert_eq!(vec[3].len(), 1);
        assert_eq!(vec[3].get(0).unwrap().test, 3);
        assert_eq!(vec.len(), 4);
    }

    #[test]
    fn test_get_element_for_6() {
        let mut vec = vec![
            Mock { test: 0 },
            Mock { test: 1 },
            Mock { test: 2 },
            Mock { test: 3 },
            Mock { test: 4 },
            Mock { test: 5 },
        ];
        let vec = split_slice_by4(&mut vec[0..6]);
        assert_eq!(vec[0].len(), 1);
        assert_eq!(vec[0].get(0).unwrap().test, 0);
        assert_eq!(vec[1].len(), 2);
        assert_eq!(vec[1].get(1).unwrap().test, 2);
        assert_eq!(vec[2].len(), 1);
        assert_eq!(vec[3].len(), 2);
        assert_eq!(vec[3].get(1).unwrap().test, 5);
        assert_eq!(vec.len(), 4);
    }
}
