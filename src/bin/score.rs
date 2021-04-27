use std::fs::{self, DirEntry, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use dissertation::metrics::Metrics;
use dissertation::{Plan, Problem};

use anyhow::{anyhow, Context};
use serde::Serialize;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    solver: PathBuf,
    problem: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
struct Record {
    // Data about the problem
    wedding: PathBuf,
    n_people: usize,
    n_tables: usize,

    // Metrics of solution quality.
    total_happiness: i64,
    mean_happiness: f64,
    median_happiness: f64,
    min_happiness: i64,
    max_happiness: i64,
    n_lonely: usize,

    // Time spent on the problem.
    // Can't use `Duration` becuase this is going into a csv.
    seconds: f64,
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

    let out_file = create_out_file(&opt.solver, &opt.problem)?;
    let mut writer = csv::Writer::from_writer(out_file);

    for record in records {
        writer.serialize(record)?;
    }

    Ok(())
}

fn create_out_file(solver: &Path, problem: &Path) -> anyhow::Result<File> {
    let solver_name = solver.file_stem().unwrap();
    let problem_name = problem.file_stem().unwrap();

    let mut csv_name = solver_name.to_owned();
    csv_name.push("_");
    csv_name.push(problem_name);

    let path = problem.with_file_name(csv_name).with_extension("csv");
    File::create(&path).with_context(move || format!("Could not create output file: {:?}", path))
}

fn score_path(solver: &Path, problem: &Path) -> anyhow::Result<Vec<Record>> {
    if problem.is_file() {
        const N_RUNS: usize = 10;

        let records: anyhow::Result<Vec<Record>> = (0..N_RUNS)
            .map(|_| {
                let wedding_file = File::open(problem)
                    .with_context(|| format!("Could not open problem file: {:?}", problem))?;
                score_single(solver, &wedding_file, problem.to_owned()).with_context(|| {
                    format!("Could not run {:?} on wedding {:?}.", solver, problem)
                })
            })
            .collect();

        records
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

fn score_single(
    solver: &Path,
    mut wedding: &File,
    wedding_name: PathBuf,
) -> anyhow::Result<Record> {
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

    // Take note of when the child started solving
    // AFTER we give them the data. We don't want to time
    // it's I/O performance.
    let time_begin = std::time::Instant::now();

    // While waiting for the solver, deserialise the problem ourselves
    // so we can evaluate the solver's performance.
    let problem_data: Problem =
        serde_json::from_slice(&problem_txt).with_context(|| "Could not deserialise problem.")?;

    // Get the solution from the child.
    let output = solver.wait_with_output()?;

    // Find out how long the child took.
    let duration = time_begin.elapsed();

    if !output.stderr.is_empty() {
        return Err(anyhow!("Solver experienced a problem."));
    }

    let plan: Plan = serde_json::from_slice(&output.stdout)
        .with_context(|| "Could not parse output from solver.")?;

    // Find out how good the solution is and return.
    let metrics = Metrics::new(&plan, &problem_data.relations);
    let score = Record {
        wedding: wedding_name,
        n_people: problem_data.relations.len(),
        n_tables: problem_data.n_tables,
        total_happiness: metrics.total_happiness(),
        mean_happiness: metrics.mean_happiness(),
        median_happiness: metrics.median_happiness(),
        min_happiness: metrics.min_happiness(),
        max_happiness: metrics.max_happiness(),
        n_lonely: metrics.n_lonely(),
        seconds: duration.as_secs_f64(),
    };
    Ok(score)
}

fn score_suite<I, E>(solver: &Path, suite: I) -> anyhow::Result<Vec<Record>>
where
    I: Iterator<Item = Result<DirEntry, E>>,
{
    // Gather the whole suite and go through it alphabetically.
    // The files are named such that alphabetical precedence => smaller problem
    // This means we can find out when things start to get ugly in performance.

    let mut entries: Vec<DirEntry> = suite.filter_map(Result::ok).collect();
    entries.sort_unstable_by_key(|entry| entry.path());

    let mut scores = Vec::with_capacity(entries.len());
    for entry in entries {
        let path_to_solve = &entry.path();
        dbg!(path_to_solve);
        scores.extend(score_path(solver, path_to_solve)?);
        dbg!(&scores);
        eprintln!();
    }

    Ok(scores)
}
