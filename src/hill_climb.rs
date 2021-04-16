use crate::metrics::Metrics;
use crate::{Plan, Problem, SeatingPlanner};

use std::{collections::VecDeque, num::NonZeroUsize};

use rand::prelude::*;

type Float = ordered_float::NotNan<f64>;

// The ratio for exponential-moving-average, which
// is used to terminate the hill-climbing algorithms.
// SAFETY: the argument to `unchecked_new` must not be NaN.
// The value is constant, so we can see it is not NaN.
// We use the unsafe version because `Float::new` is not `const`.
const EMA_FACTOR: Float = unsafe { Float::unchecked_new(0.01) };

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Swap {
    table1: usize,
    seat1: usize,
    table2: usize,
    seat2: usize,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct HillClimbingPlanner<R> {
    rng: R,
    termination_threshold: Float,
}

impl<R> HillClimbingPlanner<R>
where
    R: Rng,
{
    pub fn new(rng: R) -> Self {
        Self {
            rng,
            termination_threshold: Float::new(0.02).unwrap(),
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

        // A moving average of how often we update our best solution.
        let mut update_ema = Float::new(1.0).unwrap();

        while update_ema >= self.termination_threshold {
            // Propose a small random change.

            // TODO: if we use a priority queue (or similar) for the tables, we
            // can increase the likelihood that the most miserable person will
            // be moved.
            let swap = get_random_swap(&mut self.rng, n_tables, table_size);

            // Measure current utility.
            let old_metrics = Metrics::new(&plan, relationships);

            // Make the change and measure new utility.
            let new_metrics = Metrics::new(&plan, relationships);

            // Check if we made things better or worse.
            let updated: Float;
            if new_metrics.total_happiness() > old_metrics.total_happiness() {
                // Happy case. We found a better solution.
                updated = Float::new(1.0).unwrap();
            } else {
                // Sad case. We need to go back by performing the same swap again.
                make_swap(&mut plan, swap);
                updated = Float::new(0.0).unwrap();
            }

            update_ema = (EMA_FACTOR * updated) + ((Float::new(1.0).unwrap() - EMA_FACTOR) * update_ema);
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

        let mut update_ema = Float::new(1.0).unwrap();

        while update_ema >= self.termination_threshold {
            // Try a new solution and compare it to the front *and* back of our queue.
            let mut new_plan = queue.back().cloned().expect("nonempty queue");
            let swap = get_random_swap(&mut self.rng, n_tables, table_size);
            make_swap(&mut new_plan, swap);

            let new_metrics = Metrics::new(&new_plan, relationships);
            let new_happiness = new_metrics.total_happiness();

            let compare_to: [&Plan; 2] = [queue.front().unwrap(), queue.back().unwrap()];
            let to_update = compare_to
                .iter()
                .any(|other| new_happiness > Metrics::new(other, relationships).total_happiness());

            let updated: Float;
            if to_update {
                queue.pop_front();
                queue.push_back(new_plan);
                updated = Float::new(1.0).unwrap();
            } else {
                updated = Float::new(0.0).unwrap();
            }

            update_ema = (EMA_FACTOR * updated) + ((Float::new(1.0).unwrap() - EMA_FACTOR) * update_ema);
        }

        queue
            .into_iter()
            .max_by_key(|plan| Metrics::new(plan, &relationships).total_happiness())
            .expect("Queue length is not zero.")
    }
}

fn get_random_swap<R>(mut rng: R, n_tables: usize, table_size: usize) -> Swap
where
    R: Rng,
{
    let table1 = rng.gen_range(0..n_tables);
    let table2 = rng.gen_range(0..n_tables);

    let seat1 = rng.gen_range(0..table_size);
    let seat2 = rng.gen_range(0..table_size);

    Swap {
        table1,
        table2,
        seat1,
        seat2,
    }
}

fn make_swap(plan: &mut [Vec<usize>], swap: Swap) {
    let tmp = plan[swap.table1][swap.seat1];
    plan[swap.table1][swap.seat1] = plan[swap.table2][swap.seat2];
    plan[swap.table2][swap.seat2] = tmp;
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
