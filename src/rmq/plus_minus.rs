use std::{
    rc::Rc,
};

use crate::log::log_f;

use super::{min_index, Rmq, SparseTable};

/// Represents an RMQ algorithm for sequences that satisfy +-1 property.
/// A sequence [x_1, x_2, ..., x_n] satisfies that property if, for all i < n,
/// |x_i - x_{i + 1}| = 1. That is, consecutive elements differ by exactly 1.
/// Runtime: O(n) | O(1)
pub struct PlusMinus<T> {
    data: Rc<[T]>,

    /// States how large a single block of the data is.
    /// Defined as 1/2 * log n.
    block_size: usize,

    // Since we make the block size a power of two, we store helpers for easy
    // division and modulo operations.
    block_div: usize,
    block_mod: usize,

    /// The index of each block's minimum in the original data (B in the paper).
    block_min_idx: Vec<usize>,

    /// A RMQ to find the minimum block.
    table_rmq: SparseTable<T>,

    /// States for each block, what class it is.
    block_cls: Vec<usize>,

    // ToDo: Why is this a pointer?
    /// Allows to determine the minimum in a single block.
    class_rmq: Vec<Option<SparseTable<T>>>,
}

impl<T: PartialOrd + Copy> Rmq<T> for PlusMinus<T>
{
    fn new(data: Rc<[T]>) -> Self {
        Self {
            data,
            block_size: 0,
            block_div: 0,
            block_mod: 0,
            block_min_idx: vec![],
            table_rmq: SparseTable::new(Rc::from(vec![].into_boxed_slice())),
            block_cls: vec![],
            class_rmq: vec![],
        }
    }

    fn process_data(&mut self) {
        let data: &[T] = &self.data;
        let n = data.len();

        // Determine block size.
        {
            // The paper defines block size as 1/2 log n. We divert from that
            // and use the largest power of 2 not larger than 1/2 log n.
            // That is, we want the largest k such that
            //     2^k in [1/2 log n, 1/4 log n).
            // Note that this is equivalent to
            //     2^{k + 1} in [log n, 1/2 log n).

            let log_n = log_f(n);
            let k = log_f(log_n) - 1;

            self.block_size = 1 << k;

            self.block_div = k;
            self.block_mod = self.block_size - 1;
        }

        // --- Determine minimum in each block. ---

        // ceil(x / y) = floor((x - 1) / y) + 1
        let block_count = ((n - 1) >> self.block_div) + 1;
        {
            // The minimum of each block (A' in the paper).
            let mut block_min_val = Vec::with_capacity(block_count);

            self.block_min_idx.reserve(block_count);

            let mut b = 0;
            let mut i = 0;
            while i < n {
                // b: The current block index.
                // i: The current element in data[].

                block_min_val.push(data[i]);
                self.block_min_idx.push(i);
                i += 1;

                let cur_min = &mut block_min_val[b];
                let cur_idx = &mut self.block_min_idx[b];

                let mut j = 1;
                while j < self.block_size && i < n {
                    // j: The current index in the current block.

                    if data[i] < *cur_min {
                        *cur_min = data[i];
                        *cur_idx = i;
                    }

                    i += 1;
                    j += 1;
                }

                b += 1;
            }

            // Create RMQ over blocks.
            self.table_rmq = SparseTable::new(Rc::from(block_min_val.into_boxed_slice()));
            self.table_rmq.process_data();
        }

        // --- Classify blocks. ---

        // Note that we do not need to handle the last block B as special case as
        // long as we classify it last. If its class is unique, it will be
        // processed automatically. If it has the same class as a previous block,
        // we still only run queries on a range fitting to B which is still
        // equivalent to a query on the earlier block.

        let class_count = 1 << (self.block_size - 1);

        self.block_cls.resize(block_count, 0);
        self.class_rmq.resize_with(class_count, || None);

        for b in 0..block_count {
            let b_sta = b * self.block_size;
            let b_end = std::cmp::min(b_sta + self.block_size, n);

            let mut cls = 0;
            for i in (b_sta + 1)..b_end {
                let a = data[i - 1];
                let b = data[i];

                // Determine if it is +1 (0) or -1 (1).
                cls <<= 1;
                if a >= b {
                    cls |= 1
                }
            }

            self.block_cls[b] = cls;

            // Has that class an RMQ?
            let rmq_ptr = &mut self.class_rmq[cls];
            if rmq_ptr.is_none() {
                // Create RMQ for class.
                let mut rmq =
                    SparseTable::new(Rc::from(data[b_sta..b_end].to_vec().into_boxed_slice()));
                rmq.process_data();
                *rmq_ptr = Some(rmq);
            }
        }
    }

    fn query(&self, i: usize, j: usize) -> usize {
        // Determine block indices.
        let i_b = i >> self.block_div;
        let j_b = j >> self.block_div;

        // Determine indices in block.
        let i_idx = i & self.block_mod;
        let j_idx = j & self.block_mod;

        if i_b == j_b {
            // i and j are in the same block.
            return self.in_block_min(i_b, i_idx, j_idx);
        }

        // i and j are in different blocks.

        let i_min = self.in_block_min(i_b, i_idx, self.block_mod);
        let j_min = self.in_block_min(j_b, 0, j_idx);
        let ij_min = min_index(&self.data, i_min, j_min);

        // Are blocks adjacent?
        if i_b + 1 == j_b {
            return ij_min;
        }

        // Determine the minimum in the blocks between i and j.
        let b_idx = self.table_rmq.query(i_b + 1, j_b - 1);
        let b_min = self.block_min_idx[b_idx];

        min_index(&self.data, ij_min, b_min)
    }
}

impl<T: PartialOrd> PlusMinus<T> {
    /// Performs a query on the given block and given range.
    /// Returns the index of the minimum entry in that range with respect to the
    /// original data.
    fn in_block_min(&self, b: usize, i: usize, j: usize) -> usize {
        // Determine class and RMQ.
        let b_class = self.block_cls[b];
        let rmq = self.class_rmq[b_class].as_ref().unwrap();

        b * self.block_size /* starting point of block */ + rmq.query(i, j)
    }
}
