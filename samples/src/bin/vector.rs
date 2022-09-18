struct SmallVec<T: Default, const N: usize> {
    data: [T; N],
    size: usize,
}

impl<T: Copy + Default, const N: usize> SmallVec<T, N> {
    fn new() -> Self {
        Self {
            data: [Default::default(); N],
            size: 0,
        }
    }

    fn iter(&self) -> std::slice::Iter<T> {
        self.data[..self.size].iter()
    }

    fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.data[..self.size].iter_mut()
    }

    fn push(&mut self, elem: T) {
        if self.size >= N {
            panic!("Array too small");
        }
        self.data[self.size] = elem;
        self.size += 1;
    }
}

impl<T: Default + PartialEq, const N: usize> PartialEq for SmallVec<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

impl<T: Default + Eq, const N: usize> Eq for SmallVec<T, N> {}

impl<T: Default + Copy, const N: usize> Default for SmallVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Default + Copy, const N: usize> Extend<T> for SmallVec<T, N> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter {
            self.push(x);
        }
    }
}

impl<T: Default + Copy, const N: usize> FromIterator<T> for SmallVec<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut res: Self = Default::default();
        res.extend(iter);
        res
    }
}

impl<T: Default, const N: usize> AsRef<[T]> for SmallVec<T, N> {
    fn as_ref(&self) -> &[T] {
        &self.data[..self.size]
    }
}

impl<T: Default, const N: usize> AsMut<[T]> for SmallVec<T, N> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.data[..self.size]
    }
}

impl<'a, T: Default + Copy, const N: usize> IntoIterator for &'a SmallVec<T, N> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: Default + Copy, const N: usize> IntoIterator for &'a mut SmallVec<T, N> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

fn main() {
    let mut items: SmallVec<i32, 16> = (0..10).collect();
    for x in &mut items {
        *x *= 2;
    }
    for x in &items {
        println!("item: {}", *x);
    }
}
