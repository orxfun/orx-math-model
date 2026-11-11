use alloc::string::String;

pub type Res<T> = Result<T, Err>;

#[derive(derive_new::new)]
pub struct Err {
    msg: String,
}

impl Err {
    //
}
