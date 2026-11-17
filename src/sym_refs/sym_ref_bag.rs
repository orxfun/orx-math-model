use crate::sym_refs::SymRef;
use crate::{new_bag, Bag};

pub struct SymRefBag<'d> {
    bag: Bag<SymRef<'d>>,
}

impl Default for SymRefBag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> SymRefBag<'d> {
    pub fn push<'a, 'm, S>(&'a self, sym: S::Sym<'m, 'd>) -> &'a S::Sym<'m, 'd>
    where
        S: Sca,
    {
        let el = self.bag.push(SymRef::new::<S>(sym));
        unsafe { el.get() }.get::<S>()
    }
}
