use std::ops::Range;
use std::io;

use dissertation::GuestRelations;

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
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
    let n_guests = opt.n_tables * opt.table_size;
    let relations = create_relations(n_guests);
    let problem = Problem {
        relations,
        n_tables: opt.n_tables,
    };

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    serde_json::to_writer(&mut handle, &problem)?;
    Ok(())
}

fn create_relations(n_guests: usize) -> GuestRelations {
    let mut friend_lists = create_friend_lists(n_guests);
    friends_of_friends(&mut friend_lists);

    let mut relationships = vec![vec![0; n_guests]; n_guests];
    fill_adj_matrix(&friend_lists, 1, &mut relationships);
    GuestRelations::new(relationships)
}

fn create_friend_lists(n_guests: usize) -> Vec<Vec<usize>> {
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
