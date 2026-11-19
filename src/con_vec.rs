use crate::constants::MAX_DIM;
use crate::idxs::DIdx;
use crate::{Dim, Ptr};
use core::ops::{Deref, Index};

#[derive(Clone, Debug)]
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

    pub fn is_empty(&self) -> bool {
        self.len == 0
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

    pub fn push_to_front(&self, value: T) -> Self
    where
        T: Clone,
    {
        debug_assert!(self.len < N);
        let mut data = core::array::from_fn(|_| None);
        data[0] = Some(value);
        for i in 0..self.len {
            data[i + 1] = self.data[i].clone();
        }
        let len = self.len + 1;
        Self { len, data }
    }

    pub fn merge(&self, other: &Self) -> Self
    where
        T: Clone,
    {
        debug_assert!(self.len + other.len <= N);
        let mut len = self.len;
        let mut data = self.data.clone();
        for i in 0..other.len {
            data[len] = other.data[i].clone();
            len += 1;
        }
        Self { len, data }
    }

    pub fn map(&self, f: impl Fn(&T) -> T) -> Self {
        let mut data = core::array::from_fn(|_| None);
        for i in 0..self.len {
            data[i] = Some(f(self.data[i].as_ref().expect("i<len")));
        }
        let len = self.len;
        Self { len, data }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().take(self.len).filter_map(|x| x.as_ref())
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

pub struct DimVec<T>(ConVec<MAX_DIM, T>);

impl<T> Deref for DimVec<T> {
    type Target = ConVec<MAX_DIM, T>;

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
