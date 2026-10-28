use alloc::boxed::Box;

pub struct ParD1Fun<'d, K, T>(Box<dyn Fn(K) -> T + 'd>);
