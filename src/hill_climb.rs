use super::{lonely_guests, total_happiness, GuestRelations, Plan, SeatingPlanner};

use rand::prelude::*;

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
    fn plan(&mut self, relationships: &GuestRelations, n_tables: usize) -> Plan {
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
            let old_n_lonely = lonely_guests(&plan, relationships);
            let old_happiness = total_happiness(&plan, relationships);

            // Make the change and measure new utility.
            swap_guests(&mut plan, (table1, seat1), (table2, seat2));

            let new_n_lonely = lonely_guests(&plan, relationships);
            let new_happiness = total_happiness(&plan, relationships);

            // If we made things worse, go back.
            if new_n_lonely > old_n_lonely
                || (new_n_lonely == old_n_lonely && new_happiness < old_happiness)
            {
                swap_guests(&mut plan, (table1, seat1), (table2, seat2));
            }
        }
        plan
    }
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
