#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SetPos {
    Cat(usize),
    Range(usize),
}
