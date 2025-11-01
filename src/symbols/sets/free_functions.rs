use crate::symbols::{symbol_ref::SymbolRef, Set};

pub fn set_of<'m, const N: usize>(sets: [Set<'m>; N]) -> Set<'m> {
    assert_ne!(N, 0);
    // TODO: make sure all sets belong to the same model
    match N {
        _ => {
            let symbol: SymbolRef<'_, _> = sets[0].into();
            symbol.model.dep_set(sets)
        }
    }
}
