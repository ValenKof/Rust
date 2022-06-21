#[derive(Debug, PartialEq)]
pub struct Matrix<const N: usize, const M: usize> {
    data: [[f32; M]; N],
}

impl<const N: usize, const M: usize> Matrix<N, M> {
    pub fn new(data: [[f32; M]; N]) -> Self {
        Self { data }
    }

    pub fn transpose(&self) -> Matrix<M, N> {
        let mut res = [[0.0; N]; M];
        for col in 0..M {
            for row in 0..N {
                res[col][row] = self.data[row][col];
            }
        }
        Matrix::new(res)
    }
}

impl Matrix<4, 1> {
    pub fn to_tuple(&self) -> crate::tuple::Tuple {
        crate::tuple::Tuple::new(
            self.data[0][0],
            self.data[1][0],
            self.data[2][0],
            self.data[3][0],
        )
    }
}

impl<const N: usize, const M: usize> crate::approx::Approx for Matrix<N, M> {
    fn is_near(&self, other: &Self, eps: f32) -> bool {
        for row in 0..N {
            for col in 0..M {
                if !self.data[row][col].is_near(&other.data[row][col], eps) {
                    return false;
                }
            }
        }
        true
    }
}

impl<const N: usize, const M: usize> std::ops::Index<(usize, usize)> for Matrix<N, M> {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        &self.data[row][col]
    }
}

impl<const N: usize, const M: usize> std::ops::IndexMut<(usize, usize)> for Matrix<N, M> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        &mut self.data[row][col]
    }
}

impl<const N: usize, const M: usize, const K: usize> std::ops::Mul<&Matrix<M, K>>
    for &Matrix<N, M>
{
    type Output = Matrix<N, K>;

    fn mul(self, rhs: &Matrix<M, K>) -> Self::Output {
        let mut res = [[0.0; K]; N];
        for row in 0..N {
            for col in 0..K {
                let mut sum = 0.0;
                for i in 0..M {
                    sum += self.data[row][i] * rhs.data[i][col];
                }
                res[row][col] = sum;
            }
        }
        Self::Output::new(res)
    }
}

#[cfg(test)]
#[path = "./matrix_test.rs"]
mod tests;
