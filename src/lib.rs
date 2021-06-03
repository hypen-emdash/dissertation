mod hill_climb;

pub use hill_climb::{HillClimbingPlanner, LahcPlanner};

pub mod metrics;

use serde::{Deserialize, Serialize};

/// A complete, undirected graph that models the relationship between
/// all guests at a wedding.
/// Guests are indexed as `usize`.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
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

    /// Returns an iterator over the relationships. Should be combined with `.enumerate()`
    /// if you want the indicies of the relevant guests.
    pub fn iter(&self) -> impl Iterator<Item = impl Iterator<Item = i64> + '_> + '_ {
        self.relationships.iter().map(|row| row.iter().copied())
    }
}

/// A seating plan. The outer vector is a list of tables.
/// Each table has a list of people.
/// People are 0-indexed, using the same scheme as `GuestRelations`.
/// We use `Vec` for simplicity, but order is not for tables
/// or people within tables.
/// Each table should be the same size, and each guest should
/// appear exactly once.
pub type Plan = Vec<Vec<usize>>;

/// A problem-instance, which solvers should read from stdin with json.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Problem {
    pub relations: GuestRelations,
    pub n_tables: usize,
}

/// The main trait of the project - implement this for your solver.
pub trait SeatingPlanner {
    fn plan(&mut self, problem: &Problem) -> Plan;
}

/// Once a solver is constructed, pass it to this function to
/// read the problem and write the solution.
pub fn run<T>(mut planner: T) -> anyhow::Result<()>
where
    T: SeatingPlanner,
{
    use std::io;

    let stdin = io::stdin();
    let reader = stdin.lock();

    let stdout = io::stdout();
    let writer = stdout.lock();

    let problem: Problem = serde_json::from_reader(reader)?;
    let plan = planner.plan(&problem);

    serde_json::to_writer(writer, &plan)?;

    Ok(())
}
