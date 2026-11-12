use alloc::string::String;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicBool, Ordering};

pub unsafe fn empty_string() -> &'static String {
    static INITIALIZED: AtomicBool = AtomicBool::new(false);
    static mut STRING: MaybeUninit<String> = MaybeUninit::uninit();

    if !INITIALIZED.load(Ordering::Acquire) {
        unsafe { STRING.write(Default::default()) };
        INITIALIZED.store(true, Ordering::Release);
    }
    unsafe { STRING.assume_init_ref() }
}
