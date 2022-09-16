use crate::matrix::Matrix;
use crate::point::point;
use crate::ray::Ray;
use crate::transforms::Transform;

pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f32,
    transform: Matrix<4, 4>,
    inv_transform: Matrix<4, 4>,
    half_width: f32,
    half_height: f32,
    pixel_size: f32,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = (hsize as f32) / (vsize as f32);
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        let pixel_size = (half_width * 2.0) / (hsize as f32);
        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::identity(),
            inv_transform: Matrix::identity(),
            half_width,
            half_height,
            pixel_size,
        }
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let x = (x as f32 + 0.5) * self.pixel_size;
        let y = (y as f32 + 0.5) * self.pixel_size;
        let x = self.half_width - x;
        let y = self.half_height - y;
        let pixel = point(x, y, -1.0).apply(&self.inv_transform);
        let origin = point(0, 0, 0).apply(&self.inv_transform);
        let direction = (pixel - origin).normalized();
        Ray { origin, direction }
    }

    pub fn set_transform(&mut self, transform: Matrix<4, 4>) {
        self.transform = transform;
        self.inv_transform = self.transform.inverse().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::point;
    use crate::test_utils::*;
    use crate::transforms;
    use crate::vector::vector;
    use std::f32::consts::{FRAC_1_SQRT_2, PI};

    #[test]
    fn test_create_camera() {
        let c = Camera::new(160, 120, PI / 2.0);
        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, PI / 2.0);
        assert_eq!(c.transform, Matrix::identity());
    }

    #[test]
    fn test_horizontal_canvas_pixel_size() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn test_vertical_canvas_pixel_size() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn test_ray_through_center() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, point(0, 0, 0));
        assert_near!(r.direction, vector(0, 0, -1));
    }

    #[test]
    fn test_ray_through_corner() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin, point(0, 0, 0));
        assert_near!(r.direction, vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn test_ray_with_transformed_camera() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.set_transform(
            &transforms::rotation_y(PI / 4.0) * &transforms::translation(0.0, -2.0, 5.0),
        );
        let r = c.ray_for_pixel(100, 50);
        assert_near!(r.origin, point(0, 2, -5));
        assert_near!(r.direction, vector(FRAC_1_SQRT_2, 0.0, -FRAC_1_SQRT_2));
    }
}
