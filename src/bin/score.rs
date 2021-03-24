use std::ffi::{OsStr, OsString};
use std::fs::{File, ReadDir};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use dissertation::metrics::{lonely_guests, total_happiness};
use dissertation::{Plan, Problem};

use anyhow::{anyhow, Context};
use serde::Serialize;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    solver: OsString,
    problem: PathBuf,
}

#[derive(Debug, Copy, Clone, Serialize)]
struct Score {
    happiness: i64,
    n_lonely: usize,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    if let Err(e) = run(opt) {
        eprintln!("{}", e);
        Err(e)
    } else {
        Ok(())
    }
}

fn run(opt: Opt) -> anyhow::Result<()> {
    if opt.problem.is_file() {
        let wedding_file = File::open(&opt.problem)
            .with_context(|| format!("Could not open problem file: {:?}", &opt.problem))?;

        let score = score_wedding(&opt.solver, &wedding_file)
            .with_context(|| format!("Could not run {:?} on {:?}.", &opt.solver, &opt.problem))?;

        println!("{:?}", score);
    } else if opt.problem.is_dir() {
        todo!("Implement mass scoring for directories.");
    } else {
        return Err(anyhow!(format!(
            "Could not identify type of {:?}. Possibly a broken symlink?",
            opt.problem
        )));
    }

    Ok(())
}

fn score_wedding(solver: &OsStr, mut wedding: &File) -> anyhow::Result<Score> {
    // Create the solver as a child process.
    let mut solver = Command::new(&solver)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| "Unable to spawn solver.")?;

    // Read the problem.
    let mut problem_txt = Vec::new();
    wedding
        .read_to_end(&mut problem_txt)
        .with_context(|| "Could not read problem from file.")?;

    // Pipe the problem to the child.
    let child_stdin = solver.stdin.as_mut().expect("We gave the child a stdin.");
    child_stdin
        .write_all(&problem_txt)
        .with_context(|| "Could not pipe problem to solver.")?;
    child_stdin
        .flush()
        .with_context(|| "Could not flush solver's stdin.")?;

    // While waiting for the solver, deserialise the problem ourselves
    // so we can evaluate the solver's performance.
    let problem_data: Problem =
        serde_json::from_slice(&problem_txt).with_context(|| "Could not deserialise problem.")?;

    // Get the solution from the child.
    let output = solver.wait_with_output()?;
    if !output.stderr.is_empty() {
        return Err(anyhow!("Solver experienced a problem."));
    }
    let plan: Plan = serde_json::from_slice(&output.stdout)
        .with_context(|| "Could not parse output from solver.")?;

    // Find out how good the solution is and return.
    let score = Score {
        happiness: total_happiness(&plan, &problem_data.relations),
        n_lonely: lonely_guests(&plan, &problem_data.relations),
    };
    Ok(score)
}

fn score_suite(solver: &Path, suite: &ReadDir) -> anyhow::Result<Vec<Score>> {
    todo!()
}
