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
    let rels = create_relations(20_000);

    let popularities: Vec<i64> = rels
        .iter()
        .map(|rel_list| rel_list.sum::<i64>())
        .collect();
    println!("popularities: {:?}", popularities);
    println!("pop avg: {:?}", popularities.iter().sum::<i64>() as f64 / popularities.len() as f64);

    let problem = Problem {
        relations: rels,
        n_tables: 4,
    };
    let json = serde_json::to_string(&problem).expect("What could go wrong?");
    let reconstructed: Problem = serde_json::from_str(&json).expect("We just serialised it.");
    assert_eq!(problem, reconstructed);
}

fn create_relations(n_guests: usize) -> GuestRelations {
    let mut rng = thread_rng();

    let mut friend_lists = vec![Vec::new(); n_guests];

    // Start by assigning everyone one random friend.
    for i in 0..n_guests {
        // Give people a random number of friends, but at least one.
        let mut count = 0;
        while rng.gen_range(0..=count) == 0 {
            let j = random_associate(&mut rng, i, 0..n_guests);
            friend_lists[i].push(j);
            friend_lists[j].push(i);
            count += 1; // This will increment even if i and j are already friends, but it's approximate anyway.
        }
    }

    // Transform the adjacency matrix into an adjacency graph.
    let mut relationships = vec![vec![0; n_guests]; n_guests];
    for (guest_id, friend_list) in friend_lists.into_iter().enumerate() {
        for friend_id in friend_list {
            // The adjacency list should be symmetrical, so we don't need to
            // reflect this.
            relationships[guest_id][friend_id] = 1;
        }
    }

    GuestRelations::new(relationships)
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
