use crate::symbols::{parameters::Count, Symbol};
use core::ops::{Range, RangeBounds};

#[derive(derive_new::new, Clone, Copy)]
pub struct RangeSet<'a> {
    s: Symbol<'a>,
}

fn abc(a: Count<'_>) {
    let x = a..a;
    let b = x.start;
    let c = x.end;

    let x = a..=a;
    let b = x.start();
    let c = x.end();
}
