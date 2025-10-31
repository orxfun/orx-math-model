use super::into_set::IntoSet;
use alloc::vec::Vec;

// usize

impl IntoSet for &[usize] {
    type SetGen<'m> = Vec<usize>;

    fn into_set_gen<'m>(self) -> Self::SetGen<'m> {
        self.to_vec()
    }
}
