//! Implements logarithm functions.

/// Returns the floor of the logarithm base 2 of the given number `n`.
#[inline(always)]
pub const fn log_f_32(n: u32) -> u32 {
    31 ^ (n | 1).leading_zeros()
}

/// Returns the floor of the logarithm base 2 of the given number `n`.
#[inline(always)]
pub const fn log_f_64(n: u64) -> u32 {
    63 ^ (n | 1).leading_zeros()
}

/// Returns the floor of the logarithm base 2 of the given number `n`.
#[inline(always)]
pub const fn log_f(n: usize) -> usize {
    (8 * std::mem::size_of::<usize>() - 1) ^ (n | 1).leading_zeros() as usize
}

/// Returns the ceil of the logarithm base 2 of the given number `n`.
#[inline(always)]
pub const fn log_c_32(n: u32) -> u32 {
    log_f_32(n - 1) + 1
}

/// Returns the ceil of the logarithm base 2 of the given number `n`.
#[inline(always)]
pub const fn log_c_64(n: u64) -> u32 {
    log_f_64(n - 1) + 1
}
