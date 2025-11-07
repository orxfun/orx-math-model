use crate::symbols::sets::SetCore;
use crate::symbols::symbol_ref_core::SymbolRefCore;
use crate::Par;
use core::num::NonZeroUsize;

// TODO: make sure that this cannot be called with N = 0
/// # Panics
///
/// Panics if called with an empty array `[]`.
pub fn par<'m, const N: usize>(sets: [SetCore<'m>; N]) -> Par<'m, N> {
    // TODO: make sure all sets belong to the same model
    let _ = NonZeroUsize::new(N).expect("must be nonnegative");
    let first_symbol: SymbolRefCore<'_, _> = sets[0].into();
    first_symbol.model.par(sets)
}
