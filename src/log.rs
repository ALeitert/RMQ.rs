//! Implements logarithm functions.

/// Returns the floor of the logarithm base 2 of the given number `n`.
#[inline(always)]
pub const fn log_f(n: usize) -> usize {
    (8 * std::mem::size_of::<usize>() - 1) ^ (n | 1).leading_zeros() as usize
}

/// Returns the ceil of the logarithm base 2 of the given number `n`.
#[inline(always)]
pub const fn log_c(n: usize) -> usize {
    log_f(n - 1) + 1
}
