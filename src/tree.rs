/// Represents the ID of a tree node.
pub type NodeId = usize;

/// Represents a null pointer.
const NULL_NODE: NodeId = NodeId::MAX;

/// Represents an [Euler tour](https://en.wikipedia.org/wiki/Euler_tour_technique) of a tree.
pub struct EulerTour {
    /// The sequence of nodes visited during an Euler tour.
    pub e: Vec<NodeId>,

    /// The level (distance to root + 1) of each node in the Euler tour.
    pub l: Vec<usize>,

    /// The index of a node's last occurrence in the Euler tour.
    pub r: Vec<usize>,
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

        let mut result = EulerTour {
            e: Vec::with_capacity(2 * n - 1),
            l: Vec::with_capacity(2 * n - 1),
            r: vec![0; n],
        };

        // Helpers to compute DFS
        let mut ch_idx = vec![0; n];
        let mut stack = Vec::new();

        // Push
        stack.push(self.root);

        while !stack.is_empty() {
            let v_id = unsafe { stack.last().copied().unwrap_unchecked() };
            let c_idx = unsafe { ch_idx.get_unchecked_mut(v_id) };

            result.r[v_id] = result.e.len();
            result.e.push(v_id);
            result.l.push(stack.len());

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

        result
    }
}
