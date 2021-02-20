use petgraph::matrix_graph::MatrixGraph;
use petgraph::Undirected;

pub type GuestID = usize;
pub type Relationship = i64;

pub type Plan = Vec<Vec<GuestID>>;

pub type Relationships = MatrixGraph<GuestID, Relationship, Undirected>;

pub trait SeatingPlanner {
    fn plan(&self, relationships: &Relationships, n_tables: usize) -> Plan;
}

pub fn lonely_guests(plan: &Plan, relationships: &Relationships) -> usize {
    todo!()
}

pub fn total_happiness(plan: &Plan, relationships: &Relationships) -> Relationship {
    todo!()
}
