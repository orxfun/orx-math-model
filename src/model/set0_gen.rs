use alloc::boxed::Box;

pub struct Set0Gen {
    f: Box<dyn Fn() -> Box<dyn Iterator<Item = usize>>>,
}

impl Default for Set0Gen {
    fn default() -> Self {
        Self {
            f: Box::new(|| Box::new(core::iter::empty())),
        }
    }
}

impl Set0Gen {
    pub fn new<I: IntoIterator<Item = usize> + 'static>(
        generator: impl Fn() -> I + 'static,
    ) -> Self {
        let f: Box<dyn Fn() -> Box<dyn Iterator<Item = usize>>> =
            Box::new(move || Box::new(generator().into_iter()));
        Self { f }
    }

    pub fn generate(&self) -> impl Iterator<Item = usize> {
        (self.f)()
    }
}
