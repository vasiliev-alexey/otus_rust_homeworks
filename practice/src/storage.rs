#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
// df259bf1de5a9165b6c9be695e838028
struct Storage<'a, T> {
    inner: Vec<&'a T>,
}

impl<'a, T> Storage<'a, T> {
    fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    fn get(&self, index: usize) -> Option<&'a T> {
        self.inner.get(index).cloned()
    }

    fn push(&mut self, value: &'a T) {
        self.inner.push(value);
    }
}

// Код ниже изменять не нужно
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let (x, y) = (10, 20);
        let ref_x = {
            let mut storage = Storage::<i32>::new();
            storage.push(&x);
            storage.push(&y);
            match storage.get(0) {
                Some(ref_x) => ref_x,
                _ => unreachable!(),
            }
        };
        println!("{ref_x}"); // вы должны увидеть "10" в консоли
    }
}
