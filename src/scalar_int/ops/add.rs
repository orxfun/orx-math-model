use crate::scalar_int::{Int, IntData};
use crate::Model;
use alloc::boxed::Box;
use core::ops::Add;

impl<'a> Add<Int<'a>> for Int<'a> {
    type Output = Int<'a>;

    fn add(self, rhs: Int<'a>) -> Self::Output {
        self.0.assert_same_model(rhs.0);

        let (m, l) = (self.0.m, self.0.d);
        let r = rhs.0.d;

        let fun = |m: &Model| l.value(m) + r.value(m);
        let x = m.fun_int(Box::new(fun));
        return x;
    }
}
