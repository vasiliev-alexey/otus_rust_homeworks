// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=8744eff9a28450ac0a576e2326bcb86c
trait Area {
    fn area(&self) -> f32;
}

struct Rectangle {
    width: f32,
    height: f32,
}

impl Area for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

struct Circle {
    radius: f32,
}

impl Area for Circle {
    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }
}

// <= Реализуйте этот трейт для Circle

struct RightTriangle {
    base: f32,
    height: f32,
}

// <= ... и для RightTriangle
impl Area for RightTriangle {
    fn area(&self) -> f32 {
        self.base * self.height / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let rect = Rectangle {
            width: 100.0,
            height: 38.8,
        };
        println!("Площадь прямоугольника = {}", rect.area());

        let circle = Circle { radius: 15.0 };
        println!("Площадь круга = {}", circle.area());

        let triangle = RightTriangle {
            base: 14.0,
            height: 36.0,
        };
        println!("Площадь прямого треугольника = {}", triangle.area());
    }
}
