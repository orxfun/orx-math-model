use crate::{ptr::RefPtr, sets::SetFun};

#[derive(derive_new::new)]
pub struct IdxData<'d, K: Copy> {
    s0: RefPtr<SetFun<'d, K>>,
    value: K,
}

impl<'d, K: Copy> IdxData<'d, K> {
    pub fn read(&self) -> K {
        self.value
    }

    pub fn write(&mut self, value: K) {
        self.value = value;
    }
}
