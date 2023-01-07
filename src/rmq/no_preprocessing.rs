use std::rc::Rc;

use super::{min_index, Rmq};

/// Represents an RMQ without pre-processing by simply iterating over the given
/// range.
/// Runtime: O(1) | O(k)
pub struct NoPreprocessing<T: PartialOrd> {
    data: Rc<[T]>,
}

impl<T: PartialOrd> Rmq<T> for NoPreprocessing<T> {
    fn new(data: Rc<[T]>) -> Self {
        Self { data }
    }

    fn process_data(&mut self) {
        // Do nothing.
    }

    fn query(&self, i: usize, j: usize) -> usize {
        // First entry is default minimum.
        let mut min_idx = i;

        for idx in (i + 1)..=j {
            min_idx = min_index(&self.data, min_idx, idx);
        }

        min_idx
    }
}
