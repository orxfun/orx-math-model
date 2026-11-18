use orx_imp_vec::{Doubling, Elem, ImpVec, SplitVec};

type Pinned<T> = SplitVec<T, Doubling>;

type BagKind<T> = ImpVec<T, Pinned<T>>;

fn new_bag<T>() -> BagKind<T> {
    BagKind::new()
}

// wrapper

pub struct Bag<T>(BagKind<T>);

impl<T> Default for Bag<T> {
    fn default() -> Self {
        Self(new_bag())
    }
}

impl<T> Bag<T> {
    pub fn push<'m>(&'m self, data: T) -> Elem<'m, T> {
        self.0.push(data)
    }
}
