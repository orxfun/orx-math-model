use crate::data_structs::ElemCont;
use crate::ptr::RefPtr;
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

// data

#[derive(derive_new::new)]
pub struct SetStrData<'d> {
    fun: SetStrFun<'d>,
}

impl<'d> SetStrData<'d> {
    pub fn empty() -> Self {
        Self::new(SetStrFun::empty())
    }

    pub fn fill(&self, elements: &mut ElemCont<&'d str>) {
        self.fun.fill(elements);
    }

    pub fn update_fun(&mut self, fun: SetStrFun<'d>) {
        self.fun = fun;
    }

    pub fn update_fun_from_gen<I>(&mut self, fun: impl Fn() -> I + 'd)
    where
        I: IntoIterator<Item = &'d str>,
    {
        let fun = move |e: &mut ElemCont<&'d str>| e.fill(fun());
        self.update_fun(SetStrFun(Box::new(fun)));
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

    // pub fn idx(self) -> Element<'m, 'd> {
    //     self.m.element(self)
    // }
}
