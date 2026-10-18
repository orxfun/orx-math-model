use crate::model::symbols::{Par, Set};
use crate::model::{Set0Gen, Slots};
use core::marker::PhantomData;

pub struct ModelWithData<'data> {
    int: Slots<i64>,
    set0: Slots<Set0Gen>,
    p: PhantomData<&'data ()>,
}
