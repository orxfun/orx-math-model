use crate::elements::ElementContainer;
use crate::ptr::RefPtr;
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// fun

#[derive(derive_new::new)]
pub struct SetCoreFun<'d, T>(Box<dyn Fn(&mut ElementContainer<T>) + 'd>);

impl<T> SetCoreFun<'_, T> {
    pub fn empty() -> Self {
        Self(Box::new(|_| {}))
    }

    pub fn fill(&self, elements: &mut ElementContainer<T>) {
        self.0(elements)
    }
}

// data

#[derive(derive_new::new)]
pub struct SetCoreData<'d, T> {
    fun: SetCoreFun<'d, T>,
}

impl<'d, T> SetCoreData<'d, T> {
    pub fn empty() -> Self {
        Self::new(SetCoreFun::empty())
    }

    pub fn fill(&self, elements: &mut ElementContainer<T>) {
        self.fun.fill(elements);
    }

    pub fn update_fun(&mut self, fun: SetCoreFun<'d, T>) {
        self.fun = fun;
    }

    pub fn update_fun_from_gen<I>(&mut self, fun: impl Fn() -> I + 'd)
    where
        I: IntoIterator<Item = T>,
    {
        let fun = move |e: &mut ElementContainer<T>| e.fill(fun());
        self.update_fun(SetCoreFun(Box::new(fun)));
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct SetCore<'m, 'd, T> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, SetCoreData<'d, T>>,
}

impl<'m, 'd, T> SetCore<'m, 'd, T> {
    pub(crate) fn fun_ptr(&self) -> RefPtr<SetCoreFun<'d, T>> {
        // TODO: safety note
        unsafe { RefPtr::new(core::ptr::addr_of!((*self.el.as_ptr()).fun)) }
    }
}
