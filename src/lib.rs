/// A complete, undirected graph that models the relationship between
/// all guests at a wedding.
/// Guests are indexed as `usize`.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GuestRelations {
    // A square array, symmetrical (ie `relationships[i][j] == relationships[j][i]`)
    relationships: Vec<Vec<i64>>,
}

impl GuestRelations {
    pub fn new(relationships: Vec<Vec<i64>>) -> Self {
        // TODO: check/force symmetry and squareness.
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
    todo!()
}

pub fn total_happiness(plan: &Plan, relationships: &GuestRelations) -> i64 {
    todo!()
}
