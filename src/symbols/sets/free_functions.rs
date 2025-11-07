use crate::symbols::{symbol_ref_core::SymbolRefCore, Set};
use core::num::NonZeroUsize;

// TODO: make sure that this cannot be called with N = 0
/// # Panics
///
/// Panics if called with an empty array `[]`.
pub fn set_of<'m, const N: usize>(sets: [Set<'m, 0>; N]) -> Set<'m, N> {
    // TODO: make sure all sets belong to the same model
    let _ = NonZeroUsize::new(N).expect("must be nonnegative");
    let first_symbol: SymbolRefCore<'_, _> = sets[0].into();
    first_symbol.model.dep_set(sets)
}
