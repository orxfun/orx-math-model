use crate::symbols::{symbol_ref::SymbolRef, Set};
use core::num::NonZeroUsize;

/// # Panics
///
/// Panics if called with an empty array `[]`.
pub fn set_of<'m, const N: usize>(sets: [Set<'m>; N]) -> Set<'m> {
    // TODO: make sure all sets belong to the same model
    let _ = NonZeroUsize::new(N).expect("must be nonnegative");
    let symbol: SymbolRef<'_, _> = sets[0].into();
    symbol.model.dep_set(sets)
}
