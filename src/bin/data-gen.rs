use std::{ops::Range, str::FromStr};
use std::io;

use dissertation::GuestRelations;

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use anyhow::anyhow;

#[derive(Debug, PartialEq, Eq)]
enum GenerationMethod {
    Random,
    CompleteComponents,
    Rings,
}

impl FromStr for GenerationMethod {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rand" | "random" => Ok(GenerationMethod::Random),
            "comp" | "complete" | "complete-components" => Ok(GenerationMethod::CompleteComponents),
            "ring" | "rings" => Ok(GenerationMethod::Rings),
            _ => Err(anyhow!("Unrecognised generation method")),
        }
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
    method: GenerationMethod,
    n_tables: usize,
    table_size: usize,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
struct Problem {
    relations: GuestRelations,
    n_tables: usize,
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
    //let relations = random_relations(n_guests);
    let relations = match opt.method {
        GenerationMethod::Random => random_relations(opt.n_tables * opt.table_size),
        GenerationMethod::CompleteComponents => complete_components(opt.n_tables, opt.table_size),
        GenerationMethod::Rings => rings(opt.n_tables, opt.table_size),
    };
    let problem = Problem {
        relations,
        n_tables: opt.n_tables,
    };

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    serde_json::to_writer(&mut handle, &problem)?;
    Ok(())
}

/// Tables where everyone knows each other.
fn complete_components(n_tables: usize, table_size: usize) -> GuestRelations {
    let n_guests = n_tables * table_size;
    let mut relations = vec![vec![0; n_guests]; n_guests];

    for table in 0..n_tables {
        for i in table*table_size..(table+1)*table_size {
            for j in table*table_size..(table+1)*table_size {
                relations[i][j] = 1;
            }
        }
    }

    for i in 0..n_guests {
        relations[i][i] = 0;
    }

    GuestRelations::new(relations)
}

fn rings(n_tables: usize, table_size: usize) -> GuestRelations {
    let n_guests = n_tables * table_size;
    let mut relations = vec![vec![0; n_guests]; n_guests];

    for table in 0..n_tables {
        for i in 0..table_size {
            let j = (i + 1) % table_size;
            relations[table*table_size + i][table*table_size + j] = 1;
            relations[table*table_size + j][table*table_size + i] = 1;
        }
    }

    GuestRelations::new(relations)
}

fn random_relations(n_guests: usize) -> GuestRelations {
    let mut friend_lists = random_friend_lists(n_guests);
    friends_of_friends(&mut friend_lists);

    let mut relationships = vec![vec![0; n_guests]; n_guests];
    fill_adj_matrix(&friend_lists, 1, &mut relationships);
    GuestRelations::new(relationships)
}

fn random_friend_lists(n_guests: usize) -> Vec<Vec<usize>> {
    let mut rng = thread_rng();

    let mut friend_lists = vec![Vec::new(); n_guests];

    // Start by assigning everyone at least one random friend.
    for i in 0..n_guests {
        while rng.gen_range(0..=friend_lists[i].len()) == 0 {
            let j = random_associate(&mut rng, i, 0..n_guests);
            friend_lists[i].push(j);
            friend_lists[j].push(i);
        }
    }

    friend_lists
}

fn friends_of_friends(friend_lists: &mut [Vec<usize>]) {
    let mut rng = thread_rng();

    for i in 0..friend_lists.len() {
        // A random number of times, but with decreasing probability.
        // Not guaranteed to run even once.
        while rng.gen_range(0..=friend_lists[i].len()) == 0 {
            let mutual_friend = friend_lists[i]
                .choose(&mut rng)
                .copied()
                .expect("Everyone should have at least one friend by now.");

            // Find a friend of our mutual friend, and make them friends with the current guest.
            let new_friend = friend_lists[mutual_friend]
                .choose(&mut rng)
                .copied()
                .expect("Everyone should have at least one friend by now.");
            
            friend_lists[i].push(new_friend);
            friend_lists[new_friend].push(i);
        }
    }
}

fn random_associate<R>(mut rng: R, person: usize, choices: Range<usize>) -> usize
where
    R: Rng,
{
    loop {
        let associate = rng.gen_range(choices.clone());
        if associate != person {
            return associate;
        }
    }
}

fn fill_adj_matrix(lists: &[Vec<usize>], val: i64, matrix: &mut [Vec<i64>]) {
    for (guest_id, rel_list) in lists.iter().enumerate() {
        for &rel_id in rel_list {
            if rel_id != guest_id {
                // No-one is considered a relative of themselves.

                // The adjacency list should be symmetrical, so we don't need to
                // reflect this.
                matrix[guest_id][rel_id] = val;
            }
        }
    }
}
