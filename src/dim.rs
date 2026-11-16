use core::ops::{Deref, Index};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Dim {
    D0,
    D1,
    D2,
}

// dim - vec

pub struct DimVec<T>(ConVec<4, T>);

impl<T> Deref for DimVec<T> {
    type Target = ConVec<4, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DimVec<T> {
    pub fn new<const N: usize>(values: [T; N]) -> Self {
        Self(ConVec::new(values))
    }

    pub fn dim(&self) -> Dim {
        match self.len {
            0 => Dim::D0,
            1 => Dim::D1,
            2 => Dim::D2,
            _ => todo!(),
        }
    }
}

impl<T> Index<usize> for DimVec<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        self.get(idx).unwrap()
    }
}

// con - vec

pub struct ConVec<const N: usize, T> {
    len: usize,
    data: [Option<T>; N],
}

impl<const N: usize, T> ConVec<N, T> {
    pub fn new<const C: usize>(values: [T; C]) -> Self {
        debug_assert!(C <= 4);
        let mut data = core::array::from_fn(|_| None);
        for (i, v) in values.into_iter().enumerate() {
            data[i] = Some(v);
        }
        Self { len: C, data }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        debug_assert!(idx < self.len);
        self.data[idx].as_ref()
    }
}
