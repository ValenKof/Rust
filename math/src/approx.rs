pub trait Approx {
    fn is_near(&self, other: &Self, eps: f32) -> bool;
}

impl Approx for f32 {
    fn is_near(&self, other: &Self, eps: f32) -> bool {
        (self - other).abs() < eps
    }
}

impl<T: Approx> Approx for Option<T> {
    fn is_near(&self, other: &Self, eps: f32) -> bool {
        match (self.as_ref(), other.as_ref()) {
            (Some(a), Some(b)) => a.is_near(b, eps),
            (None, None) => true,
            (_, _) => false,
        }
    }
}
