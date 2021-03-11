use dissertation::{run, HillClimbingPlanner};

use rand::prelude::*;

fn main() -> anyhow::Result<()> {
    let solver = HillClimbingPlanner::new(thread_rng(), 10_000);
    run(solver)?;
    Ok(())
}
