use crate::symbols::sets::set_gen::IntoSetGen;
use alloc::vec::Vec;

// usize

impl IntoSetGen for &[usize] {
    type SetGen<'m> = Vec<usize>;

    fn into_set_gen<'m>(self) -> Self::SetGen<'m> {
        self.to_vec()
    }
}
