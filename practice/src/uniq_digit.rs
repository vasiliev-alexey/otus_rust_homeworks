#![allow(dead_code)]
/*
    Дана строка, состоящая только из цифровых символов. В данной строке
    есть одна цифра, которая не повторяется. Написать функцию,
    которая найдёт эту цифру и вернёт её.

    * Написать похожую функцию, но только на этот раз в данной строке
    могут присутствовать любые символы, а уникальная цифра может отсутствовать.
    Но если присутсвует, то не больше одной. Написать тесты.
*/

use std::collections::{HashMap, HashSet};
fn uniq_digit(s: &str) -> u8 {
    let unique: HashSet<char> = s.chars().collect();
    let map: HashMap<char, usize> = unique.iter().map(|&c| (c, s.matches(c).count())).collect();
    map.iter()
        .filter(|&x| x.1 == &1_usize)
        .next()
        .unwrap()
        .0
        .to_digit(10)
        .unwrap() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(uniq_digit("3"), 3);
        assert_eq!(uniq_digit("010"), 1);
        assert_eq!(uniq_digit("47343077"), 0);
        assert_eq!(uniq_digit("123454321"), 5);
        assert_eq!(uniq_digit("0987654321234567890"), 1);
        assert_eq!(uniq_digit("4444444444424444444444444"), 2);
    }
}
