use hw9::{Matrix, MatrixSet};

fn main() {
    let matrix = Matrix::<i32, 3>::new([1, 2, 3]);
    let matrix2 = Matrix::<i32, 3>::new([4, 5, 6]);
    let matrix_group = [matrix, matrix2];
    let ms = MatrixSet::new(&matrix_group);
    println!("{}", ms.sum_all_elements());
    println!("{}", ms.multiply_all_elements());
}
