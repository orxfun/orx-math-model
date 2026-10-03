const MAX_DEP_SET_SIZE: usize = 8;

pub struct DepSet {
    indices: [Option<usize>; MAX_DEP_SET_SIZE],
}
