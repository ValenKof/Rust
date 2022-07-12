use crate::matrix::Matrix;
use crate::transforms;
use crate::tuple::{cross, Tuple};

pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix<4, 4> {
    let forward = (to - from).normalize();
    let up = up.normalize();
    let left = cross(forward, up);
    let up = cross(left, forward);
    &Matrix::new([
        [left.x, left.y, left.z, 0.0],
        [up.x, up.y, up.z, 0.0],
        [-forward.x, -forward.y, -forward.z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]) * &transforms::translation(-from.x, -from.y, -from.z)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::Matrix;
    use crate::point::point;
    use crate::test_utils::*;
    use crate::transforms;
    use crate::vector::vector;

    #[test]
    fn test_view_transformation_matrix_for_default_orientation() {
        let from = point(0, 0, 0);
        let to = point(0, 0, -1);
        let up = vector(0, 1, 0);
        assert_eq!(view_transform(from, to, up), Matrix::<4, 4>::identity());
    }

    #[test]
    fn test_view_transformation_matrix_for_reversed_orientation() {
        let from = point(0, 0, 0);
        let to = point(0, 0, 1);
        let up = vector(0, 1, 0);
        assert_eq!(
            view_transform(from, to, up),
            transforms::scaling(-1., 1., -1.)
        );
    }

    #[test]
    fn test_view_transformation_moves_the_world() {
        let from = point(0, 0, 8);
        let to = point(0, 0, 0);
        let up = vector(0, 1, 0);
        assert_eq!(
            view_transform(from, to, up),
            transforms::translation(0., 0., -8.)
        );
    }

    #[test]
    fn test_arbitrary_view_transformation() {
        let from = point(1, 3, 2);
        let to = point(4, -2, 8);
        let up = vector(1, 1, 0);
        assert_near!(
            view_transform(from, to, up),
            Matrix::new([
                [-0.50709, 0.50709, 0.67612, -2.36643],
                [0.76772, 0.60609, 0.12122, -2.82843],
                [-0.35857, 0.59761, -0.71714, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ])
        );
    }
}
