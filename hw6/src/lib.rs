pub enum Item {
    First,
    Second,
    Third,
}

impl Item {
    pub fn index(&self) -> usize {
        match self {
            Item::First => 0,
            Item::Second => 1,
            Item::Third => 2,
        }
    }
}
#[derive(PartialEq)]
pub struct Tuple(u32, f32, f64);
#[derive(PartialEq)]
pub struct Array([f64; 3]);

trait Container3Elements: Default + PartialEq<Self> {
    fn is_default(&self) -> bool {
        Self::default() == *self
    }
    fn sum(&self) -> f64 {
        self.get_item(Item::First) + self.get_item(Item::Second) + self.get_item(Item::Third)
    }
    fn get_item(&self, item: Item) -> f64;
    fn set_item(&mut self, item: Item, value: f64);
}

impl Default for Tuple {
    fn default() -> Self {
        Self(0, 0.0, 0.0)
    }
}

impl Container3Elements for Tuple {
    fn get_item(&self, item: Item) -> f64 {
        match item {
            Item::First => self.0 as _,
            Item::Second => self.1 as _,
            Item::Third => self.2,
        }
    }

    fn set_item(&mut self, item: Item, value: f64) {
        match item {
            Item::First => self.0 = value as _,
            Item::Second => self.1 = value as _,
            Item::Third => self.2 = value,
        };
    }
}

impl Default for Array {
    fn default() -> Self {
        Self([0.0; 3])
    }
}

impl Container3Elements for Array {
    fn get_item(&self, item: Item) -> f64 {
        self.0[item.index()]
    }

    fn set_item(&mut self, item: Item, value: f64) {
        self.0[item.index()] = value
    }
}

#[cfg(test)]
mod tests_container {
    use super::*;

    fn check_container_default_values<T: Container3Elements>(container: &T) {
        assert_eq!(0.0, container.get_item(Item::First));
        assert_eq!(0.0, container.get_item(Item::Second));
        assert_eq!(0.0, container.get_item(Item::Third));
    }

    fn check_container_sum<T: Container3Elements>(container: &T, sum: f64) {
        assert_eq!(container.sum(), sum);
    }

    #[test]
    fn test_container_sum() {
        let arr = Array([0.0, 0.0, 0.0]);
        check_container_sum(&arr, 0.0);
        let arr = Array([1.0, 2.0, 3.0]);
        check_container_sum(&arr, 6.0);
        let tuple = Tuple(0, 0.0, 0.0);
        check_container_sum(&tuple, 0.0);
        let tuple = Tuple(1, 2.0, 3.0);
        check_container_sum(&tuple, 6.0);
    }

    #[test]
    fn test_container_default_values() {
        check_container_default_values(&Tuple::default());
        check_container_default_values(&Array::default());
    }
}

#[cfg(test)]
mod tests_array {
    use super::*;

    #[test]
    fn test_array_default_values() {
        let arr = Array::default();
        assert_eq!([0.0, 0.0, 0.0], arr.0);
    }

    #[test]
    fn test_array_is_default() {
        let arr = Array([0.0, 0.0, 0.0]);
        assert!(arr.is_default());
    }
    #[test]
    fn test_array_is_not_default() {
        let arr = Array([0.0, 0.0, 1.0]);
        assert!(!arr.is_default());
    }
    #[test]
    fn test_array_set_index() {
        let mut arr = Array([0.0, 0.0, 1.0]);
        arr.set_item(Item::First, 1.0);
        assert_eq!(arr.0[0], 1.0);
    }
    #[test]
    fn test_array_get_index() {
        let mut arr = Array([1.0, 2.0, 3.0]);
        assert_eq!(arr.get_item(Item::Third), 3.0);
        assert_eq!(arr.get_item(Item::Second), 2.0);
        assert_eq!(arr.get_item(Item::First), 1.0);
        arr.set_item(Item::First, 0.0);
        assert_eq!(arr.get_item(Item::First), 0.0);
    }
}

#[cfg(test)]
mod tests_tuple {
    use super::*;

    #[test]
    fn test_tuple_default_values() {
        let tup = Tuple::default();
        assert_eq!(0, tup.0);
        assert_eq!(0.0, tup.1);
        assert_eq!(0.0, tup.2);
    }
    #[test]
    fn test_tuple_is_default() {
        let tup = Tuple(0, 0.0, 0.0);
        assert!(tup.is_default());
    }

    #[test]
    fn test_tuple_is_not_default() {
        let tup = Tuple(0, 0.0, 1.0);
        assert!(!tup.is_default());
    }

    #[test]
    fn test_tuple_set_index() {
        let mut tup = Tuple(0, 0.0, 1.0);
        tup.set_item(Item::First, 1.0);
        assert_eq!(tup.0, 1);
        tup.set_item(Item::Second, 1.0);
        assert_eq!(tup.1, 1.0);
        assert_eq!(tup.2, 1.0);
    }

    #[test]
    fn test_tuple_get_index() {
        let mut tup = Tuple(1, 2.0, 3.0);
        assert_eq!(tup.get_item(Item::Third), 3.0);
        assert_eq!(tup.get_item(Item::Second), 2.0);
        assert_eq!(tup.get_item(Item::First), 1.0);
        tup.set_item(Item::First, 0.0);
        assert_eq!(tup.get_item(Item::First), 0.0);
    }

    #[test]
    fn test_tuple_sum() {
        let tup = Tuple(1, 2.0, 3.0);
        assert_eq!(tup.sum(), 6.0);
    }
}
