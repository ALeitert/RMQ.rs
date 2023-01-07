use std::rc::Rc;

use super::{min_index, Rmq};

/// Represents a RMQ with an almost naive preprocessing.
/// Runtime: O(n^2) | O(1)
pub struct Naive<T: PartialOrd> {
    data: Rc<[T]>,
    table: Vec<Vec<usize>>,
}

impl<T: PartialOrd> Rmq<T> for Naive<T> {
    fn new(data: Rc<[T]>) -> Self {
        Self {
            data,
            table: Vec::new(),
        }
    }

    fn process_data(&mut self) {
        let n = self.data.len();

        self.table.resize_with(n, || vec![0; n]);

        // --- Compute all results. ---

        for i in 0..n {
            // Base case.
            self.table[i][i] = i;

            for j in (i + 1)..n {
                // Recursive call.
                let min_idx = min_index(&self.data, self.table[i][j - 1], j);

                self.table[i][j] = min_idx;
                self.table[j][i] = min_idx;
            }
        }
    }

    fn query(&self, i: usize, j: usize) -> usize {
        self.table[i][j]
    }
}
