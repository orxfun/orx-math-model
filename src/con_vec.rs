use crate::Dim;
use core::ops::{Deref, Index};

#[derive(Clone)]
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

    pub fn push(&self, value: T) -> Self
    where
        T: Clone,
    {
        debug_assert!(self.len < N);
        let mut data = self.data.clone();
        data[self.len] = Some(value);
        let len = self.len + 1;
        Self { len, data }
    }
}

impl<const N: usize, T> Index<usize> for ConVec<N, T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, idx: usize) -> &Self::Output {
        self.get(idx).unwrap()
    }
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

    #[inline(always)]
    fn index(&self, idx: usize) -> &Self::Output {
        self.get(idx).unwrap()
    }
}
