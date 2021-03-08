use dissertation::*;

use rand::prelude::*;

fn main() {
    let guest_relations = spousal_partition(30);

    let n_tables = 2;

    let n_iters = 1_000;
    let mut solver = HillClimbingPlanner::new(thread_rng(), n_iters);

    let mut plan = solver.plan(&guest_relations, n_tables);
    assert_eq!(plan.len(), n_tables);

    // Makes it easier to look at in this case.
    for table in &mut plan {
        table.sort_unstable();
    }

    println!("{:?}", plan);
}

fn spousal_partition(n: usize) -> GuestRelations {
    // Initialise the plan to all zeros.
    let mut plan = vec![vec![0; n]; n];

    // Set the top-left quadrant to all ones.
    for i in 0..n / 2 {
        for j in 0..n / 2 {
            plan[i][j] = 1;
        }
    }

    // Set the bottom-right quadrant to all ones.
    for i in n / 2..n {
        for j in n / 2..n {
            plan[i][j] = 1;
        }
    }

    // No-one should consider themselves company.
    for i in 0..n / 2 {
        plan[i][i] = 0;
    }

    GuestRelations::new(plan)
}
