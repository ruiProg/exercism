use std::{
    cmp::min,
    collections::{BTreeMap, HashSet, VecDeque},
};

use num::Integer;

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    is_solvable(capacity_1, capacity_2, goal)
        .then(|| {
            let buckets_capacity =
                BucketQuantity::from([(Bucket::One, capacity_1), (Bucket::Two, capacity_2)]);
            BucketSpace::new(buckets_capacity, *start_bucket).find_solution(goal)
        })
        .flatten()
}

impl Bucket {
    fn other(&self) -> Self {
        match self {
            Self::One => Self::Two,
            Self::Two => Self::One,
        }
    }
}

type BucketQuantity = BTreeMap<Bucket, u8>;

struct Path {
    state: BucketQuantity,
    moves: u8,
}

struct BucketSpace {
    capacity: BucketQuantity,
    visited: HashSet<BucketQuantity>,
    options: VecDeque<Path>,
}

impl BucketSpace {
    fn new(capacity: BucketQuantity, start_bucket: Bucket) -> Self {
        let other_bucket = start_bucket.other();
        let start_capacity = capacity[&start_bucket];
        let other_capacity = capacity[&other_bucket];

        Self {
            capacity,
            visited: HashSet::from([BucketQuantity::from([
                (start_bucket, 0),
                (other_bucket, other_capacity),
            ])]),
            options: VecDeque::from([Path {
                state: BucketQuantity::from([(start_bucket, start_capacity), (other_bucket, 0)]),
                moves: 1,
            }]),
        }
    }

    fn find_solution(&mut self, goal: u8) -> Option<BucketStats> {
        loop {
            let Some(candidate) = self.options.pop_front() else {
                break None;
            };
            if let Some(solution) = is_solution(&candidate, goal) {
                break Some(solution);
            }

            let new_items = candidate
                .state
                .keys()
                .into_iter()
                .flat_map(|&bucket| {
                    [
                        empty_bucket(&candidate.state, bucket),
                        fill_bucket(&candidate.state, bucket, self.capacity[&bucket]),
                        pour_from(&candidate.state, bucket, self.capacity[&bucket.other()]),
                    ]
                })
                .filter_map(|new_candidate| {
                    new_candidate.filter(|new_candidate| !self.visited.contains(new_candidate))
                })
                .map(|new_candidate| Path {
                    state: new_candidate,
                    moves: candidate.moves + 1,
                });

            self.options.extend(new_items);
            self.visited.insert(candidate.state);
        }
    }
}

fn is_solvable(capacity_1: u8, capacity_2: u8, goal: u8) -> bool {
    goal > 0
        && (capacity_1 >= goal || capacity_2 >= goal)
        && goal % u8::gcd(&capacity_1, &capacity_2) == 0
}

fn is_solution(candidate: &Path, goal: u8) -> Option<BucketStats> {
    candidate
        .state
        .iter()
        .find(|&(_, &quantity)| quantity == goal)
        .map(|(&bucket, _)| BucketStats {
            moves: candidate.moves,
            goal_bucket: bucket,
            other_bucket: candidate.state[&bucket.other()],
        })
}

fn empty_bucket(previous: &BucketQuantity, bucket: Bucket) -> Option<BucketQuantity> {
    previous
        .iter()
        .all(|(_, &quantity)| quantity != 0)
        .then(|| new_quantity(previous, bucket, 0))
}

fn fill_bucket(
    previous: &BucketQuantity,
    bucket: Bucket,
    bucket_capacity: u8,
) -> Option<BucketQuantity> {
    (previous[&bucket] != bucket_capacity).then(|| new_quantity(previous, bucket, bucket_capacity))
}

fn pour_from(
    previous: &BucketQuantity,
    bucket: Bucket,
    other_bucket_capacity: u8,
) -> Option<BucketQuantity> {
    let quantity_available = previous[&bucket];
    let other_bucket = bucket.other();
    let space_available = other_bucket_capacity - previous[&other_bucket];

    (quantity_available != 0 && space_available != 0).then(|| {
        let pouring_quantity = min(quantity_available, space_available);
        BucketQuantity::from([
            (bucket, quantity_available - pouring_quantity),
            (other_bucket, previous[&other_bucket] + pouring_quantity),
        ])
    })
}

fn new_quantity(previous: &BucketQuantity, bucket: Bucket, new_value: u8) -> BucketQuantity {
    let other_bucket = bucket.other();
    BucketQuantity::from([(bucket, new_value), (other_bucket, previous[&other_bucket])])
}
