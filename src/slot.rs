use crate::{Bag, Model, Res, ResValid};
use core::marker::PhantomData;
use orx_imp_vec::Elem;

// state

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Empty,
    Bound,
}

pub struct SlotStates {
    bag: Bag<State>,
}

impl SlotStates {
    pub fn push(&self) -> Elem<'_, State> {
        self.bag.push(State::Empty)
    }
}

// slot

pub struct Slot<'d, T> {
    state: *mut State,
    p: *mut T,
    phantom: PhantomData<fn() -> &'d ()>,
}

impl<'d, T> Slot<'d, T> {
    pub fn new<'m>(m: &'m Model<'d>, el: Elem<'m, T>) -> Self {
        Self {
            state: m.slot_state().as_mut_ptr(),
            p: el.as_mut_ptr(),
            phantom: PhantomData,
        }
    }
}

impl<'d, T> Clone for Slot<'d, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'d, T> Copy for Slot<'d, T> {}

impl<'d, T> Slot<'d, T> {
    pub fn state(&self) -> State {
        *unsafe { &*self.state }
    }

    pub fn bind(&self, value: T) -> Res {
        Res::assert_slot_empty(*unsafe { &*self.state })?;
        *unsafe { &mut *(self.state) } = State::Bound;
        *unsafe { &mut *self.p } = value;
        Ok(())
    }

    pub fn assert_is_bound(&self) -> Res {
        Res::assert_slot_bound(self.state())
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.p }
    }

    pub fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.p }
    }

    pub fn mut_ptr(&self) -> *mut T {
        self.p
    }

    pub fn ptr(&self) -> Ptr<'d, T> {
        Ptr {
            p: self.p,
            phantom: PhantomData,
        }
    }

    pub fn ptr_mut(&self) -> PtrMut<'d, T> {
        PtrMut {
            p: self.p,
            phantom: PhantomData,
        }
    }
}

// ptr

pub struct Ptr<'d, T> {
    p: *const T,
    phantom: PhantomData<fn() -> &'d ()>,
}

impl<'d, T> Clone for Ptr<'d, T> {
    fn clone(&self) -> Self {
        Self {
            p: self.p,
            phantom: PhantomData,
        }
    }
}

impl<'d, T> Copy for Ptr<'d, T> {}

impl<'d, T> Ptr<'d, T> {
    pub fn new_from_el(el: Elem<'_, T>) -> Self {
        Self {
            p: el.as_ptr(),
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    pub fn get(&self) -> &'d T {
        unsafe { &*self.p }
    }

    #[inline(always)]
    pub fn val(&self) -> T
    where
        T: Copy,
    {
        *unsafe { &*self.p }
    }

    pub unsafe fn into_el<'m>(&self) -> Elem<'m, T> {
        unsafe { Elem::new(self.p as *mut T) }
    }

    pub fn ptr_eq(self, other: Self) -> bool {
        self.p == other.p
    }
}

// ptr-mut

pub struct PtrMut<'d, T> {
    p: *mut T,
    phantom: PhantomData<fn() -> &'d ()>,
}

impl<'d, T> PtrMut<'d, T> {
    #[inline(always)]
    pub fn val(&self) -> T
    where
        T: Copy,
    {
        *unsafe { &*self.p }
    }

    #[inline(always)]
    pub fn get_mut(&self) -> &'d mut T {
        unsafe { &mut *self.p }
    }
}
