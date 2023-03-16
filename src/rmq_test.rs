//! Defines functions to compare the correctness and runtime of RMQ algorithms.

use std::{mem::swap, rc::Rc, time::Instant};

use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{
    log::log_f,
    rmq::{PlusMinus, Rmq},
    tree::Tree,
};

/// The numeric type for testing.
pub type Number = i32;

/// Used as result when making runtime tests.
pub type TimePair = (i64, i64);

/// Verifies that two RMQ algorithm create the same result.
/// Randomly picks index pairs and compares the result.
pub fn verify_algorithms<S, T>(data_size: usize, queries: usize, seed: u64) -> bool
where
    S: Rmq<Number>,
    T: Rmq<Number>,
{
    // Generate random numbers.
    let mut rng = StdRng::seed_from_u64(seed);
    let data = generate_data(data_size, &mut rng);

    // Generate and test algorithms.
    let mut rmq1 = S::new(data.clone());
    let mut rmq2 = T::new(data.clone());

    rmq1.process_data();
    rmq2.process_data();

    // Verify algorithms.
    for _ in 0..queries {
        let (i, j) = random_index_pair(&mut rng, data_size);

        let min1 = rmq1.query(i, j);
        let min2 = rmq2.query(i, j);

        if data[min1] != data[min2] {
            return false;
        }
    }

    true
}

/// Determines the runtime of the given algorithm.
/// Returns the runtime for preprocessing and for queries.
pub fn get_runtime<T: Rmq<Number>>(data_size: usize, queries: usize, seed: u64) -> TimePair {
    // Generate random numbers.
    let mut rng = StdRng::seed_from_u64(seed);
    let data = generate_data(data_size, &mut rng);

    // Run test.
    let rmq = T::new(data);
    get_runtime_inner(rmq, rng, data_size, queries)
}

/// Verifies that two RMQ algorithm create the same result.
/// Randomly picks index pairs and compares the result.
pub fn verify_plus_minus<T>(data_size: usize, queries: usize, seed: u64) -> bool
where
    T: Rmq<Number>,
{
    // ToDo: Update function description.

    // Generate random numbers.
    let mut rng = StdRng::seed_from_u64(seed);
    let data = generate_plus_minus(data_size, &mut rng);

    // Generate and test algorithms.
    let mut rmq1 = PlusMinus::new(data.clone());
    let mut rmq2 = T::new(data.clone());

    rmq1.process_data();
    rmq2.process_data();

    // Verify algorithms.
    for _ in 0..queries {
        let (i, j) = random_index_pair(&mut rng, data_size);

        let min1 = rmq1.query(i, j);
        let min2 = rmq2.query(i, j);

        if data[min1] != data[min2] {
            return false;
        }
    }

    true
}

/// Determines the runtime of the given algorithm.
/// Returns the runtime for preprocessing and for queries.
pub fn get_plus_minus_runtime(data_size: usize, queries: usize, seed: u64) -> TimePair {
    // ToDo: Update function description.

    // Generate random numbers.
    let mut rng = StdRng::seed_from_u64(seed);
    let data = generate_plus_minus(data_size, &mut rng);

    // Run test.
    let rmq = PlusMinus::new(data);
    get_runtime_inner(rmq, rng, data_size, queries)
}

/// Measures the time needed to preprocess and to run queries using the
/// given RMQ algorithm.
pub fn get_ancestor_runtime<T>(tree_size: usize, queries: usize, seed: u64) -> TimePair
where
    T: Rmq<usize>,
{
    // ToDo: Implement LCA.
    todo!();

    // // Generate a random tree.
    // let mut rng = StdRng::seed_from_u64(seed);
    // let tree = generate_tree(tree_size, &mut rng);

    // let lca = Lca::new(tree);

    // // Preprocessing
    // let start = Instant::now();
    // lca.process_data();
    // let p_time = start.elapsed();

    // // Queries
    // let start = Instant::now();
    // for _ in 0..queries {
    //     let (u_id, v_id) = random_index_pair(&mut rng, tree_size);
    //     lca.query(u_id, v_id);
    // }
    // let q_time = start.elapsed();

    // (p_time, q_time)
}

/// Generates a list of random numbers with the given size.
fn generate_data<R: Rng>(size: usize, rng: &mut R) -> Rc<[Number]> {
    let max_val = (size * log_f(size)) as Number;
    let shift = max_val >> 2;

    let mut data = Vec::with_capacity(size);
    data.resize_with(size, || rng.gen_range(0..max_val) - shift);

    Rc::from(data.into_boxed_slice())
}

/// Generates a list of random numbers that satisfy the +-1 property.
fn generate_plus_minus<R: Rng>(size: usize, rng: &mut R) -> Rc<[Number]> {
    let max_val = (size * log_f(size)) as Number;
    let shift = max_val >> 2;

    let mut data = Vec::with_capacity(size);

    // Random first entry.
    data.push(rng.gen_range(0..max_val) - shift);

    let diff = [1, -1];
    for i in 1..size {
        data.push(data[i - 1] + diff[rng.gen_range(0..2)]);
    }

    Rc::from(data.into_boxed_slice())
}

/// Generates a random tree of the given size.
fn generate_tree<R: Rng>(size: usize, rng: &mut R) -> Tree {
    // Create list with node IDs.
    let mut nodes = Vec::with_capacity(size);
    for i in 0..size {
        nodes.push(i);
    }

    // Shuffle node IDs.
    for i in 0..size {
        let idx = rng.gen_range(i..size);
        nodes.swap(i, idx);
    }

    // Set parents for each node.
    let mut parents = vec![usize::MAX; size];
    for i in 1..size {
        let n_id = nodes[i];
        let p_id = nodes[rng.gen_range(0..i)];
        parents[n_id] = p_id;
    }

    Tree::from_parents(parents)
}

/// Returns two indices `i` and `j` such that `0 <= i < j < max_index`.
/// Both indices are selected using the given random number generator.
#[inline]
fn random_index_pair<R: Rng>(rng: &mut R, max_index: usize) -> (usize, usize) {
    let mut i = rng.gen_range(0..max_index);
    let mut j = rng.gen_range(0..(max_index - 1));

    // Ensure that i < j.
    if i <= j {
        j += 1;
    }

    if i > j {
        swap(&mut i, &mut j);
    }

    (i, j)
}

/// Measures the time needed to preprocess and to run queries using the
/// given RMQ algorithm.
fn get_runtime_inner<R, T>(mut rmq: T, mut rng: R, data_size: usize, queries: usize) -> TimePair
where
    T: Rmq<Number>,
    R: Rng,
{
    // Preprocessing
    let start = Instant::now();
    rmq.process_data();
    let p_time = start.elapsed();

    // Queries
    let start = Instant::now();
    for _ in 0..queries {
        let (i, j) = random_index_pair(&mut rng, data_size);
        rmq.query(i, j);
    }
    let q_time = start.elapsed();

    (p_time.as_millis() as i64, q_time.as_millis() as i64)
}
