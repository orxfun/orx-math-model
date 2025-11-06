use crate::symbols::values::set_gen::SetGen;
use alloc::boxed::Box;
use alloc::vec::Vec;

// vec

impl SetGen<0> for Vec<usize> {
    fn elements(&self, _: [usize; 0]) -> Box<dyn Iterator<Item = usize> + '_> {
        Box::new(self.iter().copied())
    }
}
