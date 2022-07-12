pub fn point<T: crate::F32Const>(x: T, y: T, z: T) -> crate::tuple::Tuple {
    crate::tuple::Tuple::point(x.to_f32(), y.to_f32(), z.to_f32())
}
