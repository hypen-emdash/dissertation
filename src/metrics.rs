use crate::{GuestRelations, Plan};

pub fn lonely_guests(plan: &Plan, relationships: &GuestRelations) -> usize {
    let mut n_lonely = 0;
    for table in plan {
        for guest1 in table {
            // A guest is lonely if they have a neutral or negative relationship with everyone else at the table.
            // It should be assumed that everyone has a neutral relationship with themself.
            if table
                .iter()
                .all(|guest2| relationships.relationship(*guest1, *guest2) <= 0)
            {
                n_lonely += 1;
            }
        }
    }

    n_lonely
}

pub fn total_happiness(plan: &Plan, relationships: &GuestRelations) -> i64 {
    let mut total = 0;
    for table in plan {
        for guest1 in table {
            for guest2 in table {
                total += relationships.relationship(*guest1, *guest2)
            }
        }
    }

    total
}
