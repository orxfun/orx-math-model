pub const ZERO_POS_EPSILON: f64 = 1e-8;

pub const ZERO_NEG_EPSILON: f64 = -ZERO_POS_EPSILON;

pub const fn is_zero(c: f64) -> bool {
    c < ZERO_POS_EPSILON && c > ZERO_NEG_EPSILON
}

pub const fn is_pos(c: f64) -> bool {
    c > ZERO_POS_EPSILON
}

pub const fn is_neg(c: f64) -> bool {
    c < ZERO_NEG_EPSILON
}

pub const MAX_EXPR_LEN: usize = 8;
pub const MAX_SUM_LEN: usize = 8;

pub const MAX_DIM: usize = 4;
