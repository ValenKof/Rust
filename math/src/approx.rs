pub trait Approx {
    fn is_near(&self, other: &Self, eps: f32) -> bool;
}

impl Approx for f32 {
    fn is_near(&self, other: &Self, eps: f32) -> bool {
        (self - other).abs() < eps
    }
}

#[macro_export]
macro_rules! assert_near {
    ($lhs:expr, $rhs:expr) => {
        assert_near!($lhs, $rhs, 1e-5)
    };
    ($lhs:expr, $rhs:expr, $eps:expr) => {
        assert!((&$lhs).is_near(&$rhs, $eps))
    };
}
