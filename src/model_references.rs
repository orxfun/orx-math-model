use crate::symbols::{Idx, Set};
use crate::Model;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SetRef {
    i: usize,
}

impl SetRef {
    pub fn new<'m, 'd>(set: Set<'m, 'd>) -> Self {
        let i = set.el.position_in(&set.m.set).expect("TODO: error msg");
        Self { i }
    }

    pub fn set<'m, 'd>(self, m: &'m Model<'d>) -> Set<'m, 'd> {
        Set::new(m, m.set.get(self.i).expect("TODO: error msg"))
    }
}

#[derive(Clone, Copy)]
pub struct IdxValRef {
    p: *const usize,
}

impl IdxValRef {
    /// # SAFETY
    ///
    /// * Created value must not outlive `'m` or `'d`.
    pub unsafe fn new<'m, 'd>(idx: Idx<'m, 'd>) -> Self {
        let el = unsafe { idx.el.get() };
        let p = el.value_ptr();
        Self { p }
    }

    /// # SAFETY
    ///
    /// This method is safe as long is it is created with the `new` method
    /// satisfying its safety requirements.
    pub fn value(self) -> usize {
        unsafe { *self.p }
    }
}
