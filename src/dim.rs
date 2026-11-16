use core::ops::Index;

#[derive(Debug, PartialEq, Clone, Copy)]
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

    pub fn dim(&self) -> Dim {
        match self.len {
            0 => Dim::D0,
            1 => Dim::D1,
            2 => Dim::D2,
            _ => todo!(),
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
