use std::ffi::{OsStr, OsString};
use std::fs::{self, DirEntry, File};
use std::io::{Read, Write};
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

#[derive(Debug, Clone, Serialize)]
struct Score {
    total_happiness: i64,
    n_lonely: usize,
}

#[derive(Debug, Clone, Serialize)]
struct Record {
    wedding: PathBuf,
    // Can't have a struct because we need to serialize.
    total_happiness: i64,
    n_lonely: usize,
}

impl Record {
    fn new(wedding: PathBuf, score: Score) -> Self {
        Self {
            wedding,
            total_happiness: score.total_happiness,
            n_lonely: score.n_lonely,
        }
    }
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
    let records = score_path(&opt.solver, &opt.problem)?;

    let out_file = File::create(opt.problem.with_extension("csv"))?;
    let mut writer = csv::Writer::from_writer(out_file);

    for record in records {
        writer.serialize(record)?;
    }

    Ok(())
}

fn score_path(solver: &OsStr, problem: &Path) -> anyhow::Result<Vec<Record>> {
    if problem.is_file() {
        let wedding_file = File::open(problem)
            .with_context(|| format!("Could not open problem file: {:?}", problem))?;

        let score = score_single(solver, &wedding_file)
            .with_context(|| format!("Could not run {:?} on wedding {:?}.", solver, problem))?;

        Ok(vec![Record::new(problem.to_owned(), score)])
    } else if problem.is_dir() {
        let entries = fs::read_dir(problem)
            .with_context(|| format!("Could not open directory {:?}.", problem))?;

        score_suite(solver, entries)
    } else {
        Err(anyhow!(format!(
            "Could not recognise {:?}. Possibly a broken symlink?",
            problem
        )))
    }
}

fn score_single(solver: &OsStr, mut wedding: &File) -> anyhow::Result<Score> {
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
        total_happiness: total_happiness(&plan, &problem_data.relations),
        n_lonely: lonely_guests(&plan, &problem_data.relations),
    };
    Ok(score)
}

fn score_suite<I, E>(solver: &OsStr, mut suite: I) -> anyhow::Result<Vec<Record>>
where
    I: Iterator<Item = Result<DirEntry, E>>,
{
    let mut scores = Vec::new();
    while let Some(Ok(entry)) = suite.next() {
        scores.extend(score_path(solver, &entry.path())?);
    }
    Ok(scores)
}
