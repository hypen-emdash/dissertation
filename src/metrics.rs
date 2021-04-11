use crate::{GuestRelations, Plan};

pub struct Metrics {
    // Index by guest to get a vector of how they feel about
    // everone *else* at the table.
    neighbour_relationships: Vec<Vec<i64>>,
}

impl Metrics {
    pub fn new(plan: &Plan, relationships: &GuestRelations) -> Self {
        let mut inner = vec![Vec::new(); relationships.len()];

        for table in plan {
            for guest in table {
                for neighbour in table {
                    if guest == neighbour {
                        continue;
                    }
                    inner[*guest].push(relationships.relationship(*guest, *neighbour))
                }
            }
        }

        Self {
            neighbour_relationships: inner,
        }
    }

    pub fn n_lonely(&self) -> usize {
        todo!()
    }

    pub fn happinesses(&self) -> impl Iterator<Item = i64> + '_ {
        self.neighbour_relationships.iter().map(|v| v.iter().sum())
    }

    pub fn total_happiness(&self) -> i64 {
        self.happinesses().sum()
    }

    pub fn mean_happiness(&self) -> f64 {
        todo!()
    }

    pub fn median_happiness(&self) -> f64 {
        todo!()
    }

    pub fn max_happiness(&self) -> i64 {
        todo!()
    }

    pub fn min_happiness(&self) -> i64 {
        todo!()
    }
}

pub fn lonely_guests(plan: &Plan, relationships: &GuestRelations) -> usize {
    let mut n_lonely = 0;
    for table in plan {
        for guest1 in table {
            // A guest is lonely if they have a neutral or negative relationship with everyone else at the table.
            // It should be assumed that everyone has a neutral relationship with themself.
            if table
                .iter()
                .all(|guest2| relationships.relationship(*guest1, *guest2) <= 0)
            {
                n_lonely += 1;
            }
        }
    }

    n_lonely
}

pub fn total_happiness(plan: &Plan, relationships: &GuestRelations) -> i64 {
    let mut total = 0;
    for table in plan {
        for guest1 in table {
            for guest2 in table {
                total += relationships.relationship(*guest1, *guest2)
            }
        }
    }

    total
}
