use crate::con_vals::ConVal;
use crate::TConVal;

#[derive(derive_new::new)]
pub struct DConVal {
    val: ConVal,
}

impl DConVal {
    pub fn get<T: TConVal>(&self) -> &T {
        self.val.read::<T>()
    }
}
