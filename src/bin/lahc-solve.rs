use dissertation::{run, LahcPlanner};

use ordered_float::NotNan;
use rand::prelude::*;

fn main() -> anyhow::Result<()> {
    let solver = LahcPlanner::new(thread_rng(), NotNan::new(0.001).unwrap());
    run(solver)
}
