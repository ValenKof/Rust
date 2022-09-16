struct Vector<T> {
    data: Vec<T>,
}

struct VectorIterator<'a, T> {
    vec: &'a Vector<T>,
    pos: usize,
}

impl<T> Vector<T> {
    fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl<T: PartialEq> PartialEq for Vector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

impl<T: Eq> Eq for Vector<T> {}

impl<T> Default for Vector<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Extend<T> for Vector<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.data.extend(iter);
    }
}

impl<T> FromIterator<T> for Vector<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut res: Self = Default::default();
        res.extend(iter);
        res
    }
}

impl<'a, T> VectorIterator<'a, T> {
    fn new(vec: &'a Vector<T>) -> VectorIterator<'a, T> {
        VectorIterator { vec, pos: 0 }
    }
}

impl<'a, T> Iterator for VectorIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.vec.data.len() {
            self.pos += 1;
            Some(&self.vec.data[self.pos - 1])
        } else {
            None
        }
    }
}

impl<'a, T> IntoIterator for &'a Vector<T> {
    type Item = &'a T;
    type IntoIter = VectorIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

fn main() {
    let items: Vector<i32> = (0..10).collect();
    for x in &items {
        println!("item: {}", x);
    }
}
