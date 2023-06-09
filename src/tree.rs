use std::rc::Rc;

use crate::rmq::Rmq;

/// Represents the ID of a tree node.
pub type NodeId = usize;

/// Represents a null pointer.
const NULL_NODE: NodeId = NodeId::MAX;

/// Represents an [Euler tour](https://en.wikipedia.org/wiki/Euler_tour_technique) of a tree.
pub struct EulerTour {
    /// The sequence of nodes visited during an Euler tour.
    pub e: Rc<[NodeId]>,

    /// The level (distance to root + 1) of each node in the Euler tour.
    pub l: Rc<[usize]>,

    /// The index of a node's last occurrence in the Euler tour.
    pub r: Rc<[usize]>,
}

/// Represents a rooted tree.
pub struct Tree {
    /// The ID of the root node.
    root: NodeId,

    /// The parents of each node.
    parents: Vec<NodeId>,

    /// The children of each node.
    children: Vec<Vec<NodeId>>,
}

impl Tree {
    /// Creates an empty tree.
    pub fn new() -> Self {
        Self {
            root: NULL_NODE,
            parents: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Creates a new tree from the given list of parents.
    pub fn from_parents(parents: Vec<NodeId>) -> Self {
        let mut children = Vec::with_capacity(parents.len());
        children.resize_with(parents.len(), Vec::new);

        let mut root = NULL_NODE;

        for (u_idx, &p_id) in parents.iter().enumerate() {
            if p_id == NULL_NODE {
                root = u_idx;
            } else {
                children[p_id].push(u_idx);
            }
        }

        Self {
            root,
            parents,
            children,
        }
    }

    /// Creates a new tree from the given slice of parents.
    pub fn from_parents_slice(parents: &[NodeId]) -> Self {
        Self::from_parents(parents.to_vec())
    }

    /// Returns the parent's ID of the given node.
    pub fn parent(&self, u_id: NodeId) -> NodeId {
        self.parents[u_id]
    }

    /// Returns the children of the given node.
    pub fn children(&self, u_id: NodeId) -> &[NodeId] {
        &self.children[u_id]
    }

    /// Computes an Euler tour of the tree.
    pub fn euler_tour(&self) -> EulerTour {
        let n = self.parents.len();

        let mut e = Vec::with_capacity(2 * n - 1);
        let mut l = Vec::with_capacity(2 * n - 1);
        let mut r = vec![0; n];

        // Helpers to compute DFS
        let mut ch_idx = vec![0; n];
        let mut stack = Vec::new();

        // Push
        stack.push(self.root);

        while !stack.is_empty() {
            let v_id = unsafe { stack.last().copied().unwrap_unchecked() };
            let c_idx = unsafe { ch_idx.get_unchecked_mut(v_id) };

            r[v_id] = e.len();
            e.push(v_id);
            l.push(stack.len());

            if *c_idx < self.children[v_id].len() {
                let child_id = self.children[v_id][*c_idx];

                // Push
                stack.push(child_id);
                *c_idx += 1;
            } else {
                // All neighbours checked, backtrack.
                stack.pop();
            }
        }

        EulerTour {
            e: Rc::from(e.into_boxed_slice()),
            l: Rc::from(l.into_boxed_slice()),
            r: Rc::from(r.into_boxed_slice()),
        }
    }
}

/// Defines an algorithm to find the lowest common ancestor of two nodes in a
/// tree using a given RMQ algorithm.
pub struct Lca<'a, T: Rmq<NodeId>> {
    tree: &'a Tree,
    et: EulerTour,
    rmq: T,
}

impl<'a, T: Rmq<NodeId>> Lca<'a, T> {
    pub fn new(tree: &'a Tree) -> Self {
        Self {
            tree,
            et: EulerTour {
                e: Rc::from(vec![].into_boxed_slice()),
                l: Rc::from(vec![].into_boxed_slice()),
                r: Rc::from(vec![].into_boxed_slice()),
            },
            rmq: T::new(Rc::from(vec![].into_boxed_slice())),
        }
    }

    pub fn process_data(&mut self) {
        self.et = self.tree.euler_tour();
        self.rmq = T::new(self.et.l.clone());
        self.rmq.process_data();
    }

    pub fn query(&self, u_id: usize, v_id: usize) -> NodeId {
        let r_u = self.et.r[u_id];
        let r_v = self.et.r[v_id];

        // Ensure that i <= j.
        let i = std::cmp::min(r_u, r_v);
        let j = std::cmp::max(r_u, r_v);

        // LCA(u, v) = E[rmq(R[u], R[v])]
        self.et.e[self.rmq.query(i, j)]
    }
}
