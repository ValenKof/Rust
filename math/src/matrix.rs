use crate::point::Point;
use crate::tuple::Tuple;
use crate::vector::Vector;

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<const N: usize, const M: usize> {
    data: [[f32; M]; N],
}

impl<const N: usize, const M: usize> Matrix<N, M> {
    pub fn new(data: [[f32; M]; N]) -> Self {
        Self { data }
    }

    pub fn from_vec(vec: Vec<Vec<f32>>) -> Self {
        let mut data = [[0.0; M]; N];
        for row in 0..N {
            for col in 0..M {
                data[row][col] = vec[row][col];
            }
        }
        Matrix { data }
    }

    pub fn zeroes() -> Self {
        Self {
            data: [[0.0; M]; N],
        }
    }

    pub fn transpose(&self) -> Matrix<M, N> {
        let mut data = [[0.0; N]; M];
        for col in 0..M {
            for row in 0..N {
                data[col][row] = self.data[row][col];
            }
        }
        Matrix { data }
    }
}

fn run_gauss<const N: usize, const K: usize>(a: &mut [[[f64; K]; N]; N]) -> Option<f64> {
    let mut det = 1.0;
    for i in 0..N {
        let mut best_row = i;
        for row in i + 1..N {
            if a[row][i][0].abs() > a[best_row][i][0].abs() {
                best_row = row;
            }
        }
        if best_row != i {
            det = -det;
            for j in 0..N {
                let tmp = a[i][j];
                a[i][j] = a[best_row][j];
                a[best_row][j] = tmp;
            }
        }

        if a[i][i][0].abs() < 1e-3 {
            return None;
        }

        let mult = 1.0 / a[i][i][0];
        det *= a[i][i][0];
        for j in 0..N {
            for x in 0..K {
                a[i][j][x] *= mult;
            }
        }

        for j in 0..N {
            if i == j {
                continue;
            }
            let mult = a[j][i][0];
            for k in 0..N {
                for x in 0..K {
                    a[j][k][x] -= a[i][k][x] * mult;
                }
            }
        }
    }
    if det.abs() < 1e-3 {
        return None;
    }
    Some(det)
}

impl<const N: usize> Matrix<N, N> {
    pub fn diagonal(diag: [f32; N]) -> Self {
        let mut data = [[0.0; N]; N];
        for i in 0..N {
            data[i][i] = diag[i];
        }
        Self { data }
    }

    pub fn identity() -> Self {
        Self::diagonal([1.0; N])
    }

    pub fn determinant(&self) -> f32 {
        let mut gauss = [[[0.0; 1]; N]; N];
        for i in 0..N {
            for j in 0..N {
                gauss[i][j][0] = self.data[i][j] as f64;
            }
        }
        run_gauss(&mut gauss).unwrap_or(0.0) as f32
    }

    pub fn inverse(&self) -> Option<Self> {
        let mut gauss = [[[0.0; 2]; N]; N];
        for i in 0..N {
            for j in 0..N {
                gauss[i][j][0] = self.data[i][j] as f64;
            }
            gauss[i][i][1] = 1.0;
        }

        if let Some(_) = run_gauss(&mut gauss) {
            let mut data = [[0.0; N]; N];
            for i in 0..N {
                for j in 0..N {
                    data[i][j] = gauss[i][j][1] as f32;
                }
            }
            Some(Self { data })
        } else {
            None
        }
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

impl std::ops::Mul<Tuple> for &Matrix<4, 4> {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Tuple {
        let rhs = Matrix::new([[rhs.x], [rhs.y], [rhs.z], [rhs.w]]);
        let res = self * &rhs;
        Tuple::new(
            res.data[0][0],
            res.data[1][0],
            res.data[2][0],
            res.data[3][0],
        )
    }
}

impl std::ops::Mul<Vector> for &Matrix<4, 4> {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector::try_from(self * Tuple::from(rhs)).unwrap()
    }
}

impl std::ops::Mul<Point> for &Matrix<4, 4> {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        Point::try_from(self * Tuple::from(rhs)).unwrap()
    }
}

#[cfg(test)]
#[path = "./matrix_test.rs"]
mod tests;
