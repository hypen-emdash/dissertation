use crate::metrics::Metrics;
use crate::{Plan, Problem, SeatingPlanner};

use std::{collections::VecDeque, num::NonZeroUsize};

use rand::prelude::*;

type Float = ordered_float::NotNan<f64>;

/// The EMA factor is the ratio for exponential-moving-average,
/// which decides how much heavily new information should be weighted
/// over our current estimates.
/// This is used to terminate the hill-climbing algorithms. We use a
/// fairly small algorithm so as not to terminate prematurely.
// SAFETY: the argument to `unchecked_new` must not be NaN.
// The value is constant, so we can see it is not NaN.
// We use the unsafe version because `Float::new` is not `const`.
//
// Technically, `unchecked_new` is deprecated in favour of `new_unchecked`
// but the only difference is the name (a good change in my opinion, since
// it is more consistent with `std`) and the replacement is only in a newer
// version of the library. I would update this code to reflect that, but
// now that the code has run and I've used it to gather results, it's more
// important to keep it as it is.
const EMA_FACTOR: Float = unsafe { Float::unchecked_new(0.01) };

/// Calculates the new exponential moving average, based on our
/// previous estimate of it, and one new value.
/// Formula taken from Operating Systems Concepts, page 209.
/// estimate = a*x + (1-a)*estimate
fn shift_ema(old_ema: Float, new_val: Float) -> Float {
    (EMA_FACTOR * new_val) + ((Float::new(1.0).unwrap() - EMA_FACTOR) * old_ema)
}

/// A swap of two people based on their locations.
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

    /// How infrequent do updates need to get before we give up?
    termination_threshold: Float,
}

impl<R> HillClimbingPlanner<R>
where
    R: Rng,
{
    pub fn new(rng: R, termination_threshold: Float) -> Self {
        Self {
            rng,
            termination_threshold,
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

        // Get an initial solution with no regard for quality.
        let mut plan = random_plan(&mut self.rng, relationships.len(), n_tables);

        // A moving average of how often we update our best solution.
        let mut update_ema = Float::new(1.0).unwrap();

        while update_ema >= self.termination_threshold {
            // Propose a small random change.
            let swap = get_random_swap(&mut self.rng, n_tables, table_size);

            // Measure current utility.
            let old_metrics = Metrics::new(&plan, relationships);

            // Make the change and measure new utility.
            // (Changing the solution in-place means that we don't have to
            // allocate a whole new solution with each iteration, but we
            // do have to keep a record of what the change was so we can
            // undo it if necessary.)
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

            // Update our estimate of how often we update the solution.
            update_ema = shift_ema(update_ema, updated);
        }
        plan
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LahcPlanner<R> {
    rng: R,

    /// How far back do we look?
    queue_size: NonZeroUsize,

    /// How infrequent do updates need to get before we give up?
    termination_threshold: Float,
}

impl<R> LahcPlanner<R>
where
    R: Rng,
{
    pub fn new(rng: R, termination_threshold: Float) -> Self {
        Self {
            rng,
            queue_size: NonZeroUsize::new(1000).unwrap(),
            termination_threshold,
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
            // (We can't avoid allocation as we did with naive hill-climbing as we need
            // our old solution whether we accept the new one or not.)
            let mut new_plan = queue.back().cloned().expect("nonempty queue");
            let swap = get_random_swap(&mut self.rng, n_tables, table_size);
            make_swap(&mut new_plan, swap);

            // Find out how good the new solution is.
            let new_metrics = Metrics::new(&new_plan, relationships);
            let new_happiness = new_metrics.total_happiness();

            // Compare the new solution to the newest and oldest in the queue.
            let compare_to: [&Plan; 2] = [queue.front().unwrap(), queue.back().unwrap()];
            let to_update = compare_to
                .iter()
                .any(|other| new_happiness > Metrics::new(other, relationships).total_happiness());

            // Keep the new solution if it is good enough, reject if not.
            let updated: Float;
            if to_update {
                queue.pop_front();
                queue.push_back(new_plan);
                updated = Float::new(1.0).unwrap();
            } else {
                updated = Float::new(0.0).unwrap();
            }

            // Update our estimate of how often we update the solution.
            update_ema = shift_ema(update_ema, updated);
        }

        // Select the best solution we have on record.
        queue
            .into_iter()
            .max_by_key(|plan| Metrics::new(plan, &relationships).total_happiness())
            .expect("Queue length is not zero.")
    }
}

/// Picks two locations out of a hat and creates the instruction to swap
/// the people there.
///
/// Technically, could swap two people at the same table, or even pick
/// exactly the same seat twice. This happens seldom enough that we
/// don't worry about it.
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

/// Swaps the people at the locations specified in the `Swap`.
///
/// We can't use `std::mem::swap` because the two locations
/// have the same lifetime and the borrow-checker would have
/// our heads. If you don't know what the borrow-checker is
/// don't worry about it.
fn make_swap(plan: &mut [Vec<usize>], swap: Swap) {
    let tmp = plan[swap.table1][swap.seat1];
    plan[swap.table1][swap.seat1] = plan[swap.table2][swap.seat2];
    plan[swap.table2][swap.seat2] = tmp;
}

/// Creates a random plan, used to initialise the hill-climbing solutions.
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
