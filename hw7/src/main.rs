fn main() {
    let arr = [2, 4, 8, 16];

    let mut n = 2;
    let nth = nth_item(&arr, &n);
    let increased = increased_by_first_item(&arr, &mut n);

    let value = {
        let values = TwoValues::new(&arr[3], increased);

        assert_eq!(*values.get_first(), 16);

        values.get_second()
    };

    assert_eq!(*value, 4);
    assert_eq!(*nth, 8);
}

fn nth_item<'a>(data: &'a [usize], n: &usize) -> &'a usize {
    &data[*n]
}

fn increased_by_first_item<'a>(data: &[usize], n: &'a mut usize) -> &'a mut usize {
    *n += data[0];
    n
}

struct TwoValues<'a> {
    first: &'a usize,
    second: &'a usize,
}

impl<'a> TwoValues<'a> {
    pub fn new(first: &'a usize, second: &'a usize) -> Self {
        Self { first, second }
    }

    pub fn get_first(&self) -> &usize {
        self.first
    }

    pub fn get_second(&self) -> &'a usize {
        self.second
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_two_values_new() {
        let values = TwoValues::new(&1, &2);
        assert_eq!(&1, values.get_first());
        assert_eq!(&2, values.get_second());
    }

    #[test]
    fn test_increased_by_first_item() {
        let arr = [500, 1, 2, 3];
        let mut n = 100_000;
        increased_by_first_item(&arr, &mut n);
        assert_eq!(n, 100_500);
    }

    #[test]
    fn test_nth_item() {
        let arr = [2, 4, 8, 16];
        assert_eq!(*nth_item(&arr, &2), 8_usize);
    }
}
