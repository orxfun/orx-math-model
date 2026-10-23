use crate::symbols::Elements;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

pub struct SetD1Data<'d> {
    f: Box<dyn Fn(usize, &mut Elements) + 'd>,
}

impl Default for SetD1Data<'_> {
    fn default() -> Self {
        Self {
            f: Box::new(|_, _| {}),
        }
    }
}

impl<'d> SetD1Data<'d> {
    pub fn new<I>(f: impl Fn(usize) -> I + 'd) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        let g = move |i1: usize, e: &mut Elements| e.fill_with(f(i1));
        Self { f: Box::new(g) }
    }

    pub fn fill(&self, i1: usize, elements: &mut Elements) {
        (self.f)(i1, elements)
    }
}

#[derive(derive_new::new, Clone, Copy)]
pub struct SetD1<'m, 'd>(Elem<'m, SetD1Data<'d>>);

impl<'m, 'd> SetD1<'m, 'd> {
    pub(crate) fn value(&self) -> &SetD1Data<'d> {
        // TODO: safety notes: we shouldn't send out the data ref to another thread
        unsafe { self.0.get() }
    }

    pub(crate) fn set(&self, value: SetD1Data<'d>) {
        self.0.set(value);
    }
}
