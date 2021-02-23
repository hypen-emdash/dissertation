use super::{GuestRelations, Plan, SeatingPlanner};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct HillClimbingPlanner;

impl SeatingPlanner for HillClimbingPlanner {
    fn plan(&self, relationships: &GuestRelations, n_tables: usize) -> Plan {
        todo!()
    }
}
