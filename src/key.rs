pub trait Key {}

impl Key for usize {}

impl<'d> Key for &'d str {}
