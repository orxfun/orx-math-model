use crate::symbols::sets::set_gen::IntoSetGen;
use alloc::vec::Vec;

// usize

impl IntoSetGen for &[usize] {
    type SetGen = Vec<usize>;

    fn into_set_gen(self) -> Self::SetGen {
        self.to_vec()
    }
}
