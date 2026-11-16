use core::ops::Index;

pub enum Dim {
    D0,
    D1,
    D2,
}

// vec

pub struct DimVec<T> {
    len: usize,
    data: [Option<T>; 4],
}

impl<T> DimVec<T> {
    pub fn new() -> Self {
        Self {
            len: 0,
            data: core::array::from_fn(|_| None),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, value: T) {
        debug_assert!(self.len < 4);
        self.data[self.len] = Some(value);
        self.len += 1;
    }

    pub fn extend<const N: usize>(&mut self, values: [T; N]) {
        for value in values {
            self.push(value);
        }
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        debug_assert!(idx < self.len);
        self.data[idx].as_ref()
    }
}

impl<T> Index<usize> for DimVec<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        self.get(idx).unwrap()
    }
}
