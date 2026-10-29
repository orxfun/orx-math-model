use crate::data_structs::ElemCont;
use crate::ptr::RefPtr;
use crate::set_and_idx::SetAndIdx;
use crate::symbols::ElStr;
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// fun

#[derive(derive_new::new)]
pub struct SetStrFun<'d>(Box<dyn Fn(&mut ElemCont<&'d str>) + 'd>);

impl<'d> SetStrFun<'d> {
    pub fn empty() -> Self {
        Self(Box::new(|_| {}))
    }

    pub fn fill(&self, elements: &mut ElemCont<&'d str>) {
        self.0(elements)
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct SetStr<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, SetStrFun<'d>>,
}

impl<'m, 'd> SetStr<'m, 'd> {
    pub(crate) fn fun_ptr(&self) -> RefPtr<SetStrFun<'d>> {
        // TODO: safety note
        unsafe { RefPtr::new(self.el.as_ptr()) }
    }

    pub fn idx(self) -> ElStr<'m, 'd> {
        self.m.el_str(self)
    }
}

// set & idx

impl<'m, 'd> SetAndIdx for SetStr<'m, 'd> {
    type Idx = ElStr<'m, 'd>;

    type Key = &'d str;
}
