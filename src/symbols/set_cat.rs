use crate::elements::ElementContainer;
use crate::ptr::RefPtr;
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// elem

#[derive(Debug, PartialEq)]
pub enum Elements<'d> {
    Usize(ElementContainer<usize>),
    Str(ElementContainer<&'d str>),
}

// fun

#[derive(derive_new::new)]
pub enum SetFun<'d> {
    Usize(Box<dyn Fn(&mut ElementContainer<usize>) + 'd>),
    Str(Box<dyn Fn(&mut ElementContainer<&'d str>) + 'd>),
}

impl<'d> SetFun<'d> {
    pub fn empty() -> Self {
        Self::Usize(Box::new(|_| {}))
    }

    pub fn create_elements(&self) -> Elements<'d> {
        match self {
            Self::Usize(_) => Elements::Usize(Default::default()),
            Self::Str(_) => Elements::Str(Default::default()),
        }
    }

    pub fn fill(&self, elements: &mut Elements<'d>) {
        match (self, elements) {
            (Self::Usize(f), Elements::Usize(e)) => f(e),
            (Self::Str(f), Elements::Str(e)) => f(e),
            _ => unreachable!("TODO: should not happen"),
        }
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct Set<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, SetFun<'d>>,
}

impl<'m, 'd> Set<'m, 'd> {
    pub(crate) fn fun_ptr(&self) -> RefPtr<SetFun<'d>> {
        // TODO: safety note
        unsafe { RefPtr::new(self.el.as_ptr()) }
    }
}
