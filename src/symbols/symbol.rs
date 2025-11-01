use crate::{stages::Stage, symbols::symbol_meta::SymbolMeta};
use alloc::string::String;
use core::{cell::UnsafeCell, ops::Deref};

pub struct Symbol<S: Stage, M: SymbolMeta> {
    pub key: Key,
    pub definition: Definition,
    // TODO: temporary clippy fix until we use the data
    #[allow(dead_code)]
    pub data: M::Data,
    phantom: core::marker::PhantomData<S>,
}

impl<S: Stage, M: SymbolMeta> Symbol<S, M> {
    pub fn new(data: M::Data) -> Self {
        Self {
            key: Default::default(),
            definition: Default::default(),
            data,
        }
    }
}

#[derive(Default)]
pub struct Key(UnsafeString);

impl Deref for Key {
    type Target = UnsafeString;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default)]
pub struct Definition(UnsafeString);

impl Deref for Definition {
    type Target = UnsafeString;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default)]
pub struct UnsafeString(UnsafeCell<String>);

impl UnsafeString {
    pub fn set(&self, value: impl Into<String>) {
        // SAFETY: Definition does not implement Send or Sync.
        // This can only be called from a sequential program without a race condition.
        // Each time we update, we entirely set the value of the field to a valid string.
        let x = unsafe { &mut *self.0.get() };
        *x = value.into();
    }

    pub fn value(&self) -> &str {
        let x = unsafe { &*self.0.get() };
        x.as_str()
    }
}

impl PartialEq<str> for UnsafeString {
    fn eq(&self, other: &str) -> bool {
        self.value().eq(other)
    }
}
