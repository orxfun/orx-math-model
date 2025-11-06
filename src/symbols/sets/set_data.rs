use crate::symbols::{sets::dep_set_indices::DependentSetIndices, values::Elements};
use core::{cell::UnsafeCell, fmt::Debug};

pub struct SetData {
    depends_on: DependentSetIndices,
    elements: UnsafeCell<Elements>,
}

impl SetData {
    pub fn new(depends_on: DependentSetIndices, elements: Elements) -> Self {
        assert_eq!(depends_on.dim(), elements.dim());
        Self {
            depends_on,
            elements: elements.into(),
        }
    }

    pub fn depends_on_indices(&self) -> &[usize] {
        self.depends_on.indices()
    }

    pub fn update_elements(&self, elements: Elements) {
        let p = unsafe { &mut *self.elements.get() };
        *p = elements;
    }
}

impl Debug for SetData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SetData")
            .field("depends_on", &self.depends_on_indices())
            .finish()
    }
}
