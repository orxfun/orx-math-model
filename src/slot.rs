use core::marker::PhantomData;
use orx_imp_vec::Elem;

enum State {
    Empty,
    Bound,
}

pub struct Slot<'d, T> {
    p: *mut T,
    state: State,
    phantom: PhantomData<fn() -> &'d ()>,
}

impl<'m, 'd, T> From<Elem<'m, T>> for Slot<'d, T> {
    fn from(value: Elem<'m, T>) -> Self {
        Self {
            p: value.as_mut_ptr(),
            state: State::Empty,
            phantom: PhantomData,
        }
    }
}

impl<'d, T> Slot<'d, T> {
    //
}
