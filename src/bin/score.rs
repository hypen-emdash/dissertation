use std::{process::{Command, Stdio}};
use std::path::PathBuf;
use std::fs::File;
use std::io::{self, Read, Write};

use dissertation::{Plan, Problem, lonely_guests, total_happiness};

use anyhow::anyhow;
use structopt::StructOpt;
use serde::Serialize;

#[derive(StructOpt)]
struct Opt {
    solver: PathBuf,
    problem: PathBuf,
}

#[derive(Serialize)]
struct Score {
    happiness: i64,
    n_lonely: usize,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    // Create the solver as a child process.
    let mut solver = Command::new(&opt.solver)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // Pipe the problem to the child.
    let mut problem_txt = Vec::new();
    File::open(opt.problem)?.read_to_end(&mut problem_txt)?;
    let child_stdin = solver.stdin.as_mut().expect("We gave the child a stdin.");
    child_stdin.write_all(&problem_txt)?;
    child_stdin.flush()?;

    // While waiting for the solver, find the actual problem.
    let problem_data: Problem = serde_json::from_slice(&problem_txt)?;

    // Get the solution from the child.
    let output = solver.wait_with_output()?;
    if !output.stderr.is_empty() {
        return Err(anyhow!("there was a problem solving the problem."));
    }
    let plan: Plan = serde_json::from_slice(&output.stdout)?;

    // Find out how good the solution is and print that.
    let score = Score {
        happiness: total_happiness(&plan, &problem_data.relations),
        n_lonely: lonely_guests(&plan, &problem_data.relations),
    };
    serde_json::to_writer_pretty(io::stdout().lock(), &score)?;

    Ok(())
}

/*
fn pipe<R, W>(mut reader: R, mut writer: W) -> io::Result<()>
where
    R: BufRead,
    W: Write,
{
    loop {
        let buffer = reader.fill_buf()?;
        if buffer.len() == 0 {
            break;
        }

        writer.write(buffer)?;

        let len = buffer.len();
        reader.consume(len);
    }
    Ok(())
}
*/
