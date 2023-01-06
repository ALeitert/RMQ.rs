use std::marker::PhantomData;

/// Represents an RMQ algorithms.
pub trait Rmq<T> {
    /// Constructor.
    fn new(data: &[T]) -> Self;

    /// Pre-processes the data to allow queries.
    fn process_data(&self);

    /// Performs a query on the given data and given range.
    /// Returns the index of the minimum in that range.
    /// Behaviour is undefined if the given range is invalid or pre-processing
    /// has not been done.
    fn query(&self, i: usize, j: usize) -> usize;
}

/// The reference "algorithm" which does nothing.
/// We use it to determine the overhead needed to generate test cases and call queries.
struct ReferenceRmq<T> {
    // No data needed.
    phantom_data: PhantomData<T>,
}

impl<T> Rmq<T> for ReferenceRmq<T> {
    fn new(_data: &[T]) -> Self {
        Self {
            phantom_data: PhantomData,
        }
    }

    fn process_data(&self) {
        // Do nothing.
    }

    fn query(&self, _: usize, _: usize) -> usize {
        // Do nothing.
        0
    }
}
