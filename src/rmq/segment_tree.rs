use std::rc::Rc;

use super::{min_index, Rmq};

/// Represents a node of a SegmentTree.
struct Node {
    /// The index of the node's left child.
    left: usize,

    /// The index of the node's right child.
    right: usize,

    /// The first index (inclusive) of the node's covered range.
    fr_idx: usize,

    /// The last index (inclusive) of the node's covered range.
    to_idx: usize,

    /// The index in the node's covered range with minimum value in that range.
    min_idx: usize,
}

impl Node {
    fn new() -> Self {
        Self {
            left: usize::MAX,
            right: usize::MAX,
            fr_idx: 0,
            to_idx: 0,
            min_idx: 0,
        }
    }
}

/// Represents a RMQ that uses a segment tree to run queries.
/// Runtime: O(n) | O(log n)
pub struct SegmentTree<T: PartialOrd> {
    data: Rc<[T]>,
    tree: Vec<Node>,
}

impl<T: PartialOrd> Rmq<T> for SegmentTree<T> {
    fn new(data: Rc<[T]>) -> Self {
        Self {
            data,
            tree: Vec::new(),
        }
    }

    fn process_data(&mut self) {
        let n = self.data.len();

        //
        // --- Build tree. ---

        // Determine size.
        let mut tree_size = n;
        let mut lay_sz = n;
        while lay_sz > 1 {
            lay_sz = (lay_sz + 1) >> 1;
            tree_size += lay_sz
        }

        self.tree.resize_with(tree_size, Node::new);

        // Build bottom layer.
        for i in 0..n {
            let node = &mut self.tree[tree_size - n + i];
            node.min_idx = i;
            node.fr_idx = i;
            node.to_idx = i;
        }

        // Build tree bottom-up.
        let mut q_size = n;
        let mut q_start = tree_size - n;
        while q_size > 1 {
            // Outer loop has one iteration per layer.

            //  q_size: The number of nodes in the previous layer.
            // q_start: The index of the first (i.e. left-most) node of the
            //          previous layer in tree[].

            let (upper_layers, lower_layers) = self.tree.split_at_mut(q_start);

            let cur_lay_size = (q_size + 1) >> 1;

            let mut n_ptr = q_start - cur_lay_size;
            let mut q_ptr = 0;
            while q_ptr < q_size {
                // The inner loop iterates over the nodes in the layer below and
                // "creates" the nodes of the current layer.

                // n_ptr: The index in upper_layers[] of the "new" node.
                // q_ptr: The index in lower_layers[] of the new node's first child.

                let left_node = &lower_layers[q_ptr];

                let node = &mut upper_layers[n_ptr];
                node.left = q_start + q_ptr;
                node.fr_idx = left_node.fr_idx;
                node.to_idx = left_node.to_idx;
                node.min_idx = left_node.min_idx;

                // Still one more element?
                if q_ptr + 1 < q_size {
                    let right_node = &lower_layers[q_ptr + 1];
                    node.right = q_start + q_ptr + 1;
                    node.to_idx = right_node.to_idx;
                    node.min_idx = min_index(&self.data, node.min_idx, right_node.min_idx);
                }

                n_ptr += 1;
                q_ptr += 2;
            }

            q_size = cur_lay_size;
            q_start -= q_size;
        }
    }

    fn query(&self, i: usize, j: usize) -> usize {
        let mut min_idx = i;
        let mut node_idx = 0;

        // Go down until paths to i and j split.
        loop {
            let node = &self.tree[node_idx];

            if node.fr_idx == i && node.to_idx == j {
                // Base case.
                return node.min_idx;
            }

            let left_to = self.tree[node.left].to_idx;

            if j <= left_to {
                // Go left.
                node_idx = node.left;
            } else if i > left_to {
                // Go right.
                node_idx = node.right;
            } else {
                // Split paths.
                break;
            }
        }

        // Go down left and search for i.
        let mut i_node_idx = self.tree[node_idx].left;
        loop {
            let i_node = &self.tree[i_node_idx];

            if i_node.fr_idx == i {
                // Base case.
                min_idx = min_index(&self.data, min_idx, i_node.min_idx);
                break;
            }

            if i <= self.tree[i_node.left].to_idx {
                // Get minimum from right node ...
                min_idx = min_index(&self.data, min_idx, self.tree[i_node.right].min_idx);

                // ... and go left.
                i_node_idx = i_node.left;
            } else {
                // Go right.
                i_node_idx = i_node.right;
            }
        }

        // Go down right and search for j.
        let mut j_node_idx = self.tree[node_idx].right;
        loop {
            let j_node = &self.tree[j_node_idx];

            if j_node.to_idx == j {
                // Base case.
                min_idx = min_index(&self.data, min_idx, j_node.min_idx);
                break;
            }

            let left_child = &self.tree[j_node.left];
            if j <= left_child.to_idx {
                // Go left.
                j_node_idx = j_node.left;
            } else {
                // Get minimum from left node ...
                min_idx = min_index(&self.data, min_idx, left_child.min_idx);

                // ... and go right.
                j_node_idx = j_node.right;
            }
        }

        min_idx
    }
}
