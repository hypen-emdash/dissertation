use dissertation::*;

use rand::prelude::*;

fn main() {
    // Four guests who know each other in two pairs.
    let guest_relations = GuestRelations::new(vec![
        vec![0, 1, 0, 0],
        vec![1, 0, 0, 0],
        vec![0, 0, 0, 1],
        vec![0, 0, 1, 0],
    ]);

    let n_tables = 2;

    let n_iters = 10;
    let mut solver = HillClimbingPlanner::new(thread_rng(), n_iters);

    let plan = solver.plan(&guest_relations, n_tables);
    println!("{:?}", plan);
}