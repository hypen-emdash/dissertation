use super::{GuestRelations, Plan, SeatingPlanner};

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
        let plan = random_plan(&mut self.rng, relationships.len(), n_tables);
        // TODO: improve on `plan`.
        plan
    }
}

fn random_plan<R>(mut rng: R, n_guests: usize, n_tables: usize) -> Plan
where
    R: Rng,
{
    assert!(n_guests % n_tables == 0);

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
