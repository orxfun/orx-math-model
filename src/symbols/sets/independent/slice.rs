use super::into_set::IntoSet;
use alloc::vec::Vec;

// usize

impl IntoSet for &[usize] {
    type SetGen = Vec<usize>;

    fn into_set(self) -> Self::SetGen {
        self.to_vec()
    }
}
