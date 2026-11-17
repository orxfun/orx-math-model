use crate::{Model, Ptr};
use core::mem::transmute;
use orx_imp_vec::Elem;

#[derive(derive_new::new)]
#[new(visibility = "pub(crate)")]
pub struct Symbol<'m, 'd, T> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, T>,
}

impl<'m, 'd, T> Clone for Symbol<'m, 'd, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'m, 'd, T> Copy for Symbol<'m, 'd, T> {}

impl<'m, 'd, T> Symbol<'m, 'd, T> {
    pub(crate) unsafe fn create_sym_ref(self) -> Symbol<'d, 'd, T> {
        // TODO: this is dangerous
        unsafe { transmute(Self::new(self.m, self.el)) }
    }
}

// helpers

pub(crate) trait SymWrap<'m, 'd: 'm>: Sized {
    type Data;

    fn sym(self) -> Symbol<'m, 'd, Self::Data>;

    fn m(self) -> &'m Model<'d> {
        self.sym().m
    }

    fn data(self) -> &'m Self::Data {
        unsafe { self.sym().el.get() }
    }

    fn ptr(self) -> Ptr<'d, Self::Data> {
        Ptr::new_from_el(self.sym().el)
    }
}
