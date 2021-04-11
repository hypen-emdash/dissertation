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
        self.neighbour_relationships
            .iter()
            .filter(|v| v.iter().all(|r| *r <= 0))
            .count()
    }

    pub fn happinesses(&self) -> impl Iterator<Item = i64> + '_ {
        self.neighbour_relationships.iter().map(|v| v.iter().sum())
    }

    pub fn total_happiness(&self) -> i64 {
        self.happinesses().sum()
    }

    pub fn mean_happiness(&self) -> f64 {
        self.total_happiness() as f64 / self.neighbour_relationships.len() as f64
    }

    pub fn median_happiness(&self) -> f64 {
        let all: Vec<i64> = self.happinesses().collect();

        let left_i = all.len() - 2;
        let right_i = all.len() - left_i;

        let left_med = all[left_i] as f64;
        let right_med = all[right_i] as f64;

        0.5 * (left_med + right_med)
    }

    pub fn max_happiness(&self) -> i64 {
        self.happinesses()
            .max()
            .expect("Expected a nonempty Metrics object.")
    }

    pub fn min_happiness(&self) -> i64 {
        self.happinesses()
            .min()
            .expect("Expected a nonempty Metrics object.")
    }
}
