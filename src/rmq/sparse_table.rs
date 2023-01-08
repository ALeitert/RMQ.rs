use std::rc::Rc;

use crate::log::log_f;

use super::{min_index, Rmq};

// ToDo: Description
/// Runtime: O(n log n) | O(1)
pub struct SparseTable<T> {
    data: Rc<[T]>,

    /// Table with minimums in various ranges.
    table: Vec<Vec<usize>>,
}

impl<T: PartialOrd> Rmq<T> for SparseTable<T> {
    fn new(data: Rc<[T]>) -> Self {
        Self {
            data,
            table: Vec::new(),
        }
    }

    fn process_data(&mut self) {
        let data: &[T] = &self.data;
        let n = data.len();

        // Height of the table is floor(log n) + 1
        let table_height = log_f(n) + 1;

        // We divert from the paper and have the height as first index and
        // length as second. That way, we always use the same vector instead
        // of two different ones; thereby improving caching and improving the
        // runtime slightly.
        self.table.resize_with(table_height, || vec![0; n]);

        for i in 0..n {
            self.table[0][i] = i;
        }

        for j in 1..table_height {
            for i in 0..n {
                // Compare the two ranges below:
                // M[j - 1, i] and M[j - 1, i + 2^{j - 1} - 1]

                // Check that right index is not out of range.
                let l_idx = i;
                let r_idx = std::cmp::min(n - 1, i + (1 << (j - 1)));

                let l_min = self.table[j - 1][l_idx];
                let r_min = self.table[j - 1][r_idx];

                self.table[j][i] = min_index(&self.data, l_min, r_min);
            }
        }
    }

    fn query(&self, i: usize, j: usize) -> usize {
        // k = floor(log (j − i))
        let k = log_f(j - i);

        // M[k, i]
        // M[k, j − 2^k + 1]

        let min_1 = self.table[k][i];
        let min_2 = self.table[k][j - (1 << k) + 1];

        min_index(&self.data, min_1, min_2)
    }
}
