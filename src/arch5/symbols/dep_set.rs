use crate::symbols::SetPos;

const MAX_DEP_SET_SIZE: usize = 8;

pub struct DepSet {
    indices: [Option<SetPos>; MAX_DEP_SET_SIZE],
}
