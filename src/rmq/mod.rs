use std::{marker::PhantomData, rc::Rc};

mod naive;
mod no_preprocessing;

pub use naive::Naive;
pub use no_preprocessing::NoPreprocessing;

/// Represents an RMQ algorithms.
pub trait Rmq<T> {
    /// Constructor.
    fn new(data: Rc<[T]>) -> Self;

    /// Pre-processes the data to allow queries.
    fn process_data(&mut self);

    /// Performs a query on the given data and given range.
    /// Returns the index of the minimum in that range.
    /// Behaviour is undefined if the given range is invalid or pre-processing
    /// has not been done.
    fn query(&self, i: usize, j: usize) -> usize;
}

/// Determines which of these indices stores the smaller value.
#[inline]
fn min_index<T: PartialOrd>(data: &[T], i: usize, j: usize) -> usize {
    if data[i] < data[j] {
        i
    } else {
        j
    }
}

/// The reference "algorithm" which does nothing.
/// We use it to determine the overhead needed to generate test cases and call queries.
pub struct Reference<T> {
    // No data needed.
    phantom_data: PhantomData<T>,
}

impl<T> Rmq<T> for Reference<T> {
    fn new(_data: Rc<[T]>) -> Self {
        Self {
            phantom_data: PhantomData,
        }
    }

    fn process_data(&mut self) {
        // Do nothing.
    }

    fn query(&self, _: usize, _: usize) -> usize {
        // Do nothing.
        0
    }
}
