use std::ops::Range;

use dissertation::GuestRelations;

use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
struct Problem {
    relations: GuestRelations,
    n_tables: usize,
}

fn main() {
    let friend_lists = create_friend_lists(1000);
    for (i, list) in friend_lists.iter().enumerate() {
        println!("{}: {:?}", i, list);
    }
    println!("avg pop: {}", friend_lists.iter().map(|list| list.len()).sum::<usize>() as f64 / friend_lists.len() as f64);
}

fn create_relations(n_guests: usize) -> GuestRelations {
    let friend_lists = create_friend_lists(n_guests);

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
        for rel_id in rel_list {
            // The adjacency list should be symmetrical, so we don't need to
            // reflect this.
            matrix[guest_id][*rel_id] = val;
        }
    }
}
