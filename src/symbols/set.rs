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

#[derive(derive_new::new)]
pub struct Set<'m, 'd>(Elem<'m, SetData<'d>>);

impl<'m, 'd> Set<'m, 'd> {
    pub(crate) fn value(&self) -> &SetData<'d> {
        // TODO: safety notes: we shouldn't send out the data ref to another thread
        unsafe { self.0.get() }
    }

    pub(crate) fn set(&self, value: SetData<'d>) {
        self.0.set(value);
    }
}
