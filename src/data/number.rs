pub trait Number: Clone + Copy {}

impl Number for usize {}

impl Number for u64 {}

impl Number for u32 {}

impl Number for isize {}

impl Number for i64 {}

impl Number for i32 {}

impl Number for f64 {}

impl Number for f32 {}
