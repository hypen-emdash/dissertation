use std::ops::Range;

use dissertation::GuestRelations;

use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
struct Problem {
    relations: GuestRelations,
    n_tables: usize,
}

fn main() {
    let r = create_relations(20);
    let problem = Problem {
        relations: r,
        n_tables: 4,
    };
    println!("{}", serde_json::to_string(&problem).unwrap());
}

fn create_relations(n_guests: usize) -> GuestRelations {
    let mut rng = thread_rng();

    let mut relationships = vec![vec![0; n_guests]; n_guests];

    // Start by assigning everyone one random friend.
    for i in 0..n_guests {
        let j = random_associate(&mut rng, i, 0..n_guests);
        set_relationship(&mut relationships, i, j, 1);
    }

    GuestRelations::new(relationships)
}

fn random_associate<R>(mut rng: R, person: usize, choices: Range<usize>) -> usize where R: Rng {
    loop {
        let associate = rng.gen_range(choices.clone());
        if associate != person {
            return associate;
        }
    }
}

fn set_relationship(graph: &mut [Vec<i64>], guest1: usize, guest2: usize, val: i64) {
    if guest1 != guest2 {
        graph[guest1][guest2] = val;
        graph[guest2][guest1] = val;
    }
}
