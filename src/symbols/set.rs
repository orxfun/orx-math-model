use alloc::boxed::Box;
use alloc::vec::Vec;
use orx_imp_vec::Elem;

#[derive(Default)]
pub struct Elements {
    data: Vec<usize>,
}

impl Elements {
    pub fn fill_with(&mut self, values: impl IntoIterator<Item = usize>) {
        self.data.clear();
        self.data.extend(values);
    }

    pub fn as_slice(&self) -> &[usize] {
        &self.data
    }
}

pub struct SetData<'d> {
    f: Box<dyn Fn(&mut Elements) + 'd>,
}

impl Default for SetData<'_> {
    fn default() -> Self {
        Self {
            f: Box::new(|_| {}),
        }
    }
}

impl<'d> SetData<'d> {
    pub fn new<I>(f: impl Fn() -> I + 'd) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        let g = move |e: &mut Elements| e.fill_with(f());
        Self { f: Box::new(g) }
    }

    pub fn fill(&self, elements: &mut Elements) {
        (self.f)(elements)
    }
}

#[derive(derive_new::new, Clone, Copy)]
pub struct Set<'m, 'd>(pub(crate) Elem<'m, SetData<'d>>);
