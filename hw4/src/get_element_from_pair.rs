pub fn get_element_from_pair<T>(values: &mut (T, T), is_next: bool) -> &mut T {
    if is_next {
        &mut values.1
    } else {
        &mut values.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_element() {
        struct Mock {
            test: usize,
        }
        let a = Mock { test: 0 };
        let b = Mock { test: 1 };
        let mut values = (a, b);
        let first = get_element_from_pair(&mut values, false);
        first.test += 1;
        assert_eq!(first.test, 1);
        let second = get_element_from_pair(&mut values, true);
        second.test += 2;
        assert_eq!(second.test, 3);
    }
}
