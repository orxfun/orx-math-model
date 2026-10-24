use crate::elements::Elements;
use crate::ptr::RefPtr;
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// elem

pub enum ElementsCat<'d> {
    Usize(Elements<usize>),
    Str(Elements<&'d str>),
}

// fun

#[derive(derive_new::new)]
pub enum SetCatFun<'d> {
    Usize(Box<dyn Fn(&mut Elements<usize>) + 'd>),
    Str(Box<dyn Fn(&mut Elements<&'d str>) + 'd>),
}

impl<'d> SetCatFun<'d> {
    pub fn empty() -> Self {
        Self::Usize(Box::new(|_| {}))
    }

    pub fn create_elements(&self) -> ElementsCat<'d> {
        match self {
            Self::Usize(_) => ElementsCat::Usize(Default::default()),
            Self::Str(_) => ElementsCat::Str(Default::default()),
        }
    }

    pub fn fill(&self, elements: &mut ElementsCat<'d>) {
        match (self, elements) {
            (Self::Usize(f), ElementsCat::Usize(e)) => f(e),
            (Self::Str(f), ElementsCat::Str(e)) => f(e),
            _ => unreachable!("TODO: should not happen"),
        }
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct SetCat<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, SetCatFun<'d>>,
}

impl<'m, 'd> SetCat<'m, 'd> {
    pub(crate) fn fun_ptr(&self) -> RefPtr<SetCatFun<'d>> {
        // TODO: safety note
        unsafe { RefPtr::new(self.el.as_ptr()) }
    }
}
