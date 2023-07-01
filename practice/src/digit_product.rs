#![allow(dead_code)]
/*
    Написать функцию, которая будет вычислять произведение цифр числа,
    при это цифра 0 игнорируется. Затем повторить операцию с результатом
    произведения, пока не получится число, состоящее из одной цифры.
*/

fn digit_product(n: u32) -> u8 {
    let next = n
        .to_string()
        .chars()
        .filter(|x| x != &'0')
        .map(|x| x.to_digit(10).unwrap())
        .product();
    return if n < 10 { n as u8 } else { digit_product(next) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(digit_product(0), 0);
        assert_eq!(digit_product(9), 9);
        assert_eq!(digit_product(10), 1);
        assert_eq!(digit_product(987), 2);
        assert_eq!(digit_product(123456), 4);
        assert_eq!(digit_product(123454321), 6);
    }
}
