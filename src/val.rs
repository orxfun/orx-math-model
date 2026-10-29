use crate::symbols::{Int, Num};

pub trait Val: Default {
    type Sca<'m, 'd>
    where
        'd: 'm;
}

impl Val for i32 {
    type Sca<'m, 'd>
        = Int<'m, 'd>
    where
        'd: 'm;
}

impl Val for f64 {
    type Sca<'m, 'd>
        = Num<'m, 'd>
    where
        'd: 'm;
}
