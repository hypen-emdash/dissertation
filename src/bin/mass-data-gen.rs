use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command};

use anyhow::Context;
use rand::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    suite: PathBuf,
}

// Take a text file of specifications for weddings and create
// the actual data, placed in a folder located in the same place
// and with a similar name to the spec file.
fn main() -> anyhow::Result<()> {
    let Opt { suite } = Opt::from_args();

    let spec_file = File::open(&suite)?;
    let target_dir_path = create_target_dir(&suite)?;

    mass_generate_data(&spec_file, &target_dir_path)?;

    Ok(())
}

fn create_target_dir(suite: &Path) -> io::Result<PathBuf> {
    let mut target_dir_path = suite.parent().map(ToOwned::to_owned).unwrap_or_default();
    target_dir_path.push(suite.file_stem().expect("Please name your suite suitably."));

    if let Err(e) = fs::create_dir(&target_dir_path) {
        if e.kind() != io::ErrorKind::AlreadyExists {
            return Err(e);
        }
    }

    Ok(target_dir_path)
}

fn mass_generate_data(spec_file: &File, target_dir: &Path) -> anyhow::Result<()> {
    let specs = read_specs(spec_file)?;

    let processes = specs
        .iter()
        .map(|spec| {
            let mut filename = OsString::new();
            let mut command = Command::new("./target/release/data-gen");
            for word in spec.split_whitespace() {
                command.arg(word);
                // Pad numbers in the filenames with 0s so alphabetical ordering coincides with size.
                match word.parse::<usize>() {
                    Ok(n) => filename.push(format!("{:03}", n)),
                    Err(_) => filename.push(word),
                }
                filename.push("_");
            }
            filename.push(thread_rng().next_u32().to_string());
            filename.push(".txt");
            command.arg(target_dir.join(filename));
            command
                .spawn()
                .with_context(|| format!("unable to spawn process for {}", spec))
        })
        .collect::<Result<Vec<Child>, anyhow::Error>>()?;

    for mut proc in processes {
        proc.wait()?;
    }

    Ok(())
}

fn read_specs(file: &File) -> io::Result<Vec<String>> {
    BufReader::new(file)
        .lines()
        .filter(|maybe_line| {
            maybe_line
                .as_ref()
                .map(|line| !line.is_empty())
                .unwrap_or(true)
        })
        .collect()
}
