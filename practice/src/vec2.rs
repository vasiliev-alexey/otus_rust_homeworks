// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=df259bf1de5a9165b6c9be695e838028
#![allow(dead_code)]
use std::ops::{Add, Neg};

// Трейты, которые мы используем:
// - Add (сложение) https://doc.rust-lang.org/stable/std/ops/trait.Add.html
// - Neg (отрицание) https://doc.rust-lang.org/stable/std/ops/trait.Neg.html

// PartialEq автоматически реализует операцию сравнения (==)
#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2<T> {
    x: T,
    y: T,
}

// Реализуйте Add для всех Vec2<T>, где T: Add<Output=T>
// Шаблон реализации для Vec2<f32> дан для примера, измените его.
impl<T> Add for Vec2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Neg for Vec2<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

// Реализуйте Neg для всех Vec2<T> аналогичным образом.

// Реализация `.length()` более сложна, так как у нас нет трейта Sqrt.
// Для выполнения задания достаточно реализовать length для Vec2<f32>
impl Vec2<f32> {
    fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// Дальше начинаются тесты – их менять не нужно

#[cfg(test)]
mod tests {
    use super::*;

    fn vec2f(x: f32, y: f32) -> Vec2<f32> {
        Vec2 { x, y }
    }

    fn vec2i(x: i32, y: i32) -> Vec2<i32> {
        Vec2 { x, y }
    }

    #[test]
    fn test_add() {
        let res = vec2f(1.0, 4.0) + vec2f(-9.0, 6.0);
        assert_eq!(res, vec2f(-8.0, 10.0));

        let res = vec2i(1, 4) + vec2i(-9, 6);
        assert_eq!(res, vec2i(-8, 10));
    }

    #[test]
    fn test_neg() {
        let res = -vec2f(1.0, -4.0);
        assert_eq!(res, vec2f(-1.0, 4.0));

        let res = -vec2i(1, -4);
        assert_eq!(res, vec2i(-1, 4));
    }

    #[test]
    fn test_length() {
        let res = vec2f(4.0, 3.0).length();
        assert_eq!(res, 5.0);
    }
}
