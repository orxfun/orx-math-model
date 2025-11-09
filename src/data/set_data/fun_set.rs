use orx_self_or::SoR;

pub struct FunSet<'d, Data, I, T, F>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
    F: Fn(&'d Data, &[usize]) -> I,
{
    data: &'d Data,
    fun: F,
}

impl<'d, Data, I, T, F> FunSet<'d, Data, I, T, F>
where
    I: IntoIterator<Item = T>,
    T: SoR<usize>,
    F: Fn(&'d Data, &[usize]) -> I,
{
    pub fn d0(
        data: &'d Data,
        fun2: impl Fn(&'d Data) -> I,
    ) -> FunSet<'d, Data, I, T, impl Fn(&'d Data, &[usize]) -> I> {
        let fun = move |data: &'d Data, _: &[usize]| fun2(data);
        FunSet { data, fun }
    }
}
