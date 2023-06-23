#![allow(dead_code)]
/*
    Дан массив, который содержит n неповторяющихся чисел в диапазоне
    от 0 до n включительно.

    Написать функцию, которая вернёт единственное число, отсутствующее
    в данном массиве.

    Гарантируется, что числа в массиве не повторяются и все принадлежат
    заданному диапазону.
*/

use std::collections::HashSet;

fn missing_num(nums: &[i32]) -> i32 {
    let mut set: HashSet<i32> = HashSet::new();
    for num in nums {
        set.insert(*num);
    }
    for i in 0..nums.len() {
        if !set.contains(&(i as i32)) {
            return i as i32;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(missing_num(&[1, 2]), 0);
        assert_eq!(missing_num(&[1, 0, 4, 2]), 3);
        assert_eq!(missing_num(&[0, 4, 2, 5, 3, 6]), 1);
    }
}
