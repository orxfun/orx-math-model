use alloc::boxed::Box;

pub type FNum<'d> = Box<dyn Fn() -> f64 + 'd>;
