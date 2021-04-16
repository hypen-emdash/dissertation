use dissertation::{run, LahcPlanner};

use rand::prelude::*;

fn main() -> anyhow::Result<()> {
    let solver = LahcPlanner::new(thread_rng());
    run(solver)
}
