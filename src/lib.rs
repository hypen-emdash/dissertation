use petgraph::matrix_graph::MatrixGraph;
use petgraph::Undirected;

pub type GuestID = usize;
pub type Relationship = i64;

pub type Plan = Vec<Vec<GuestID>>;

pub type Relationships = MatrixGraph<GuestID, Relationship, Undirected>;

pub trait SeatingPlanner {
    fn plan(&self, relationships: Relationships, n_tables: usize) -> Plan;
}

pub struct Score {
    pub n_lonely: usize,
    pub happiness: Relationship,
}

pub fn quality(plan: Plan, relationships: Relationships) -> Score {
    todo!()
}