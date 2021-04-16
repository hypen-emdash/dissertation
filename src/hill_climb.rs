use crate::metrics::Metrics;
use crate::{Plan, Problem, SeatingPlanner};

use std::{collections::VecDeque, num::NonZeroUsize};

use rand::prelude::*;

type Float = ordered_float::NotNan<f64>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct HillClimbingPlanner<R> {
    rng: R,
    iter_lim: u64,
}

impl<R> HillClimbingPlanner<R>
where
    R: Rng,
{
    pub fn new(rng: R, iteration_limit: u64) -> Self {
        Self {
            rng,
            iter_lim: iteration_limit,
        }
    }
}

impl<R> SeatingPlanner for HillClimbingPlanner<R>
where
    R: Rng,
{
    fn plan(&mut self, problem: &Problem) -> Plan {
        let relationships = &problem.relations;
        let n_tables = problem.n_tables;
        let table_size = relationships.len() / n_tables;

        let mut plan = random_plan(&mut self.rng, relationships.len(), n_tables);
        for _ in 0..self.iter_lim {
            // Propose a small random change.

            // TODO: if we use a priority queue (or similar) for the tables, we
            // can increase the likelihood that the most miserable person will
            // be moved.
            let table1 = self.rng.gen_range(0..n_tables);
            let table2 = self.rng.gen_range(0..n_tables);

            let seat1 = self.rng.gen_range(0..table_size);
            let seat2 = self.rng.gen_range(0..table_size);

            // Measure current utility.
            let old_metrics = Metrics::new(&plan, relationships);

            // Make the change and measure new utility.
            swap_guests(&mut plan, (table1, seat1), (table2, seat2));
            let new_metrics = Metrics::new(&plan, relationships);

            // If we made things worse, go back.
            if new_metrics.total_happiness() < old_metrics.total_happiness() {
                swap_guests(&mut plan, (table1, seat2), (table2, seat2));
            }
        }
        plan
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LahcPlanner<R> {
    rng: R,
    // How far back do we look?
    queue_size: NonZeroUsize,
    // How infrequent do updates need to get before we give up?
    termination_threshold: Float,
}

impl<R> LahcPlanner<R>
where
    R: Rng,
{
    pub fn new(rng: R) -> Self {
        Self {
            rng,
            queue_size: NonZeroUsize::new(1000).unwrap(),
            termination_threshold: Float::new(0.02).unwrap(),
        }
    }
}

impl<R> SeatingPlanner for LahcPlanner<R>
where
    R: Rng,
{
    fn plan(&mut self, problem: &Problem) -> Plan {
        let relationships = &problem.relations;
        let n_tables = problem.n_tables;
        let table_size = relationships.len() / n_tables;

        // Initialise our queue full of random solutions.
        let mut queue = VecDeque::with_capacity(self.queue_size.get());
        for _ in 0..self.queue_size.get() {
            queue.push_back(random_plan(&mut self.rng, relationships.len(), n_tables))
        }

        // Measure frequency of updating solutions.
        let mut n_tries = queue.len();
        let mut n_updates = queue.len();

        while n_updates as f64 >= self.termination_threshold.into_inner() * n_tries as f64 {
            // Try a new solution and compare it to the front *and* back of our queue.
            let mut new_plan = queue.back().cloned().expect("nonempty queue");
            random_change(&mut self.rng, &mut new_plan, n_tables, table_size);
            let new_metrics = Metrics::new(&new_plan, relationships);
            let new_happiness = new_metrics.total_happiness();

            let compare_to: [&Plan; 2] = [queue.front().unwrap(), queue.back().unwrap()];
            let to_update = compare_to
                .iter()
                .any(|other| new_happiness > Metrics::new(other, relationships).total_happiness());

            if to_update {
                queue.pop_front();
                queue.push_back(new_plan);
                n_updates += 1;
            }

            n_tries += 1;
        }

        queue
            .into_iter()
            .max_by_key(|plan| Metrics::new(plan, &relationships).total_happiness())
            .expect("Queue length is not zero.")
    }
}

fn random_change<R>(mut rng: R, plan: &mut Plan, n_tables: usize, table_size: usize)
where
    R: Rng,
{
    let table1 = rng.gen_range(0..n_tables);
    let table2 = rng.gen_range(0..n_tables);

    let seat1 = rng.gen_range(0..table_size);
    let seat2 = rng.gen_range(0..table_size);

    swap_guests(plan, (table1, seat1), (table2, seat2));
}

fn swap_guests(
    plan: &mut [Vec<usize>],
    (table1, seat1): (usize, usize),
    (table2, seat2): (usize, usize),
) {
    let tmp = plan[table1][seat1];
    plan[table1][seat1] = plan[table2][seat2];
    plan[table2][seat2] = tmp;
}

fn random_plan<R>(mut rng: R, n_guests: usize, n_tables: usize) -> Plan
where
    R: Rng,
{
    assert_eq!(n_guests % n_tables, 0);

    // Generate a random permutation of guests.
    let mut permutation = (0..n_guests).collect::<Vec<usize>>();
    permutation.shuffle(&mut rng);

    // Chunk the guests into tables.
    let table_size = n_guests / n_tables;
    permutation
        .chunks_exact(table_size)
        .map(ToOwned::to_owned)
        .collect::<Vec<Vec<usize>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_random_init() {
        let n_tables = 12;
        let table_size = 5;
        let n_guests = n_tables * table_size;
        let plan = random_plan(thread_rng(), n_guests, n_tables);

        // Correct number of tables.
        assert_eq!(plan.len(), n_tables);

        // Correct number of guests at each table.
        for table in &plan {
            assert_eq!(table.len(), table_size);
        }

        // Check that each guest appears exactly once.
        let mut guest_appearances = vec![0; n_guests];
        for table in &plan {
            for guest in table {
                guest_appearances[*guest] += 1;
            }
        }
        for n in guest_appearances {
            assert_eq!(n, 1);
        }
    }
}
