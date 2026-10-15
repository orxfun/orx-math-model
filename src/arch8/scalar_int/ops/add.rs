use crate::scalar_int::{Int, IntData};
use crate::Model;
use alloc::boxed::Box;
use core::ops::Add;

impl<'a> Add<Int<'a>> for Int<'a> {
    type Output = Int<'a>;

    fn add(self, rhs: Int<'a>) -> Self::Output {
        self.0.assert_same_model(rhs.0);

        let (m, l) = (self.0.m, self.0.d as *const IntData);
        let r = rhs.0.d as *const IntData;

        // SAFETY: TODO
        let fun = move |m: &Model| unsafe { &*l }.value(m) + unsafe { &*r }.value(m);
        m.fun_int(Box::new(fun))
    }
}

impl<'a> Add<i64> for Int<'a> {
    type Output = Int<'a>;

    fn add(self, rhs: i64) -> Self::Output {
        let (m, l) = (self.0.m, self.0.d as *const IntData);

        // SAFETY: TODO
        let fun = move |m: &Model| unsafe { &*l }.value(m) + rhs;
        m.fun_int(Box::new(fun))
    }
}

impl<'a> Add<Int<'a>> for i64 {
    type Output = Int<'a>;

    fn add(self, rhs: Int<'a>) -> Self::Output {
        let (m, r) = (rhs.0.m, rhs.0.d as *const IntData);

        // SAFETY: TODO
        let fun = move |m: &Model| self + unsafe { &*r }.value(m);
        m.fun_int(Box::new(fun))
    }
}
