use std::ops::{Add, Mul};

pub struct Matrix<T, const N: usize> {
    elements: [T; N],
}

impl<T: Copy, const N: usize> Matrix<T, N> {
    pub fn new(elements: [T; N]) -> Self {
        Self { elements }
    }
}

impl<T: Add<Output = T> + Copy, const N: usize> Matrix<T, N> {
    pub fn add(&mut self, value: T) {
        for i in 0..N {
            self.elements[i] = self.elements[i] + value;
        }
    }
}

impl<T: Mul<Output = T> + Copy, const N: usize> Matrix<T, N> {
    pub fn multiply(&mut self, value: T) {
        for i in 0..N {
            self.elements[i] = self.elements[i] * value;
        }
    }
}

pub struct MatrixSet<'a, T, const N: usize> {
    matrices: &'a [Matrix<T, N>],
}
impl<'a, T, const N: usize> MatrixSet<'a, T, N> {
    pub fn new(matrices: &'a [Matrix<T, N>]) -> Self {
        Self { matrices }
    }
}
impl<'a, T, const N: usize> MatrixSet<'a, T, N> {
    pub fn get_matrix(&self, index: usize) -> &'a Matrix<T, N> {
        &self.matrices[index]
    }
}

impl<'a, T: Add<Output = T> + Copy + Default, const N: usize> MatrixSet<'a, T, N> {
    pub fn sum_all_elements(&self) -> T {
        let mut sum: T = T::default();
        for matrix in self.matrices {
            for element in matrix.elements.iter() {
                sum = sum + *element;
            }
        }
        sum
    }
}

impl<'a, T: Mul<Output = T> + Copy + std::ops::Div<Output = T>, const N: usize>
    MatrixSet<'a, T, N>
{
    pub fn multiply_all_elements(&self) -> T {
        let mut product = self.matrices[0].elements[0];
        for matrix in self.matrices {
            for element in matrix.elements.iter() {
                product = product * *element;
            }
        }
        product / self.matrices[0].elements[0]
    }
}

#[cfg(test)]
mod unit_tests_matrix {
    use super::Matrix;

    #[test]
    fn test_matrix_new() {
        let matrix = Matrix::<i32, 3>::new([1, 2, 3]);
        assert_eq!(matrix.elements, [1, 2, 3]);
    }

    #[test]
    fn test_matrix_add() {
        let mut matrix = Matrix::<u32, 3>::new([1, 2, 3]);
        matrix.add(10);
        assert_eq!(matrix.elements, [11, 12, 13]);
    }
    #[test]
    fn test_matrix_bound_add_and_multiply() {
        let matrix = Matrix::<char, 3>::new(['a', 'b', 'c']);
        // matrix.add(10);
        // matrix.multiply(10);
        assert_eq!(matrix.elements, ['a', 'b', 'c']);
    }
    #[test]
    fn test_matrix_multiply() {
        let mut matrix1 = Matrix::<i32, 4>::new([1, 2, 3, 4]);
        matrix1.multiply(2);
        assert_eq!(matrix1.elements, [2, 4, 6, 8]);
    }
}
#[cfg(test)]
mod unit_tests_matrix_set {
    use crate::{Matrix, MatrixSet};

    #[test]
    fn test_matrix_set_new() {
        let matrix1 = Matrix::<i32, 3>::new([1, 2, 3]);
        let matrix2 = Matrix::<i32, 3>::new([4, 5, 6]);
        let binding = [matrix1, matrix2];
        let matrix_set = MatrixSet::new(&binding);
        assert_eq!(matrix_set.get_matrix(0).elements, [1, 2, 3]);
        assert_eq!(matrix_set.get_matrix(1).elements, [4, 5, 6]);
    }

    #[test]
    fn test_matrix_set_sum_all_elements() {
        let matrix1 = Matrix::<i32, 3>::new([1, 2, 3]);
        let matrix2 = Matrix::<i32, 3>::new([4, 5, 6]);
        let binding = [matrix1, matrix2];
        let matrix_set = MatrixSet::new(&binding);
        assert_eq!(matrix_set.sum_all_elements(), 1 + 2 + 3 + 4 + 5 + 6);
    }

    #[test]
    fn test_matrix_set_multiply_all_elements() {
        let matrix1 = Matrix::<i32, 3>::new([2, 1, 3]);
        let matrix2 = Matrix::<i32, 3>::new([4, 5, 6]);
        let binding = [matrix1, matrix2];
        let matrix_set = MatrixSet::new(&binding);
        assert_eq!(matrix_set.multiply_all_elements(), 1 * 2 * 3 * 4 * 5 * 6);
    }
}
