mod hill_climb;

/// A complete, undirected graph that models the relationship between
/// all guests at a wedding.
/// Guests are indexed as `usize`.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GuestRelations {
    // A square array, symmetrical (ie `relationships[i][j] == relationships[j][i]`)
    // with zeros along the diagonal.
    relationships: Vec<Vec<i64>>,
}

impl GuestRelations {
    pub fn new(relationships: Vec<Vec<i64>>) -> Self {
        // TODO: check/force symmetry, squareness, and lack of self-loops.
        Self::new_unchecked(relationships)
    }

    pub fn new_unchecked(relationships: Vec<Vec<i64>>) -> Self {
        Self { relationships }
    }

    /// Returns the degree of friendship two guests have.
    /// Positive is good, negative is bad. 0 is either unmet or self.
    /// # Panics
    /// Panics if either guest is unknown (out of bounds).
    pub fn relationship(&self, guest1: usize, guest2: usize) -> i64 {
        self.relationships[guest1][guest2]
    }

    /// Returns the number of guests.
    pub fn len(&self) -> usize {
        self.relationships.len()
    }
}

pub type Plan = Vec<Vec<usize>>;

pub trait SeatingPlanner {
    fn plan(&self, relationships: &GuestRelations, n_tables: usize) -> Plan;
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
