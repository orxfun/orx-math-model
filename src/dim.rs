use core::ops::Index;

pub enum Dim {
    D0,
    D1,
    D2,
}

impl Dim {
    pub(crate) fn nth(n: usize) -> Self {
        match n {
            0 => Self::D0,
            1 => Self::D1,
            2 => Self::D2,
            _ => unreachable!(),
        }
    }
}

// vec

pub struct DimVec<T> {
    len: usize,
    data: [Option<T>; 4],
}

impl<T> DimVec<T> {
    pub fn new<const N: usize>(values: [T; N]) -> Self {
        debug_assert!(N <= 4);
        let mut data = core::array::from_fn(|_| None);
        for (i, v) in values.into_iter().enumerate() {
            data[i] = Some(v);
        }
        Self { len: N, data }
    }

    pub fn len(&self) -> usize {
        self.len
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
