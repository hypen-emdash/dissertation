use dissertation::{run, HillClimbingPlanner};

use ordered_float::NotNan;
use rand::prelude::*;

fn main() -> anyhow::Result<()> {
    let solver = HillClimbingPlanner::new(thread_rng(), NotNan::new(0.001).unwrap());
    run(solver)
}
