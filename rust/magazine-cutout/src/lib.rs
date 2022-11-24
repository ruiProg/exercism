// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::{hash_map::Entry, HashMap};

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut occurrences = HashMap::new();

    for &word in note {
        let count = occurrences.entry(word).or_insert(0);
        *count += 1;
    }

    for &word in magazine {
        if let Entry::Occupied(mut entry) = occurrences.entry(word) {
            let remaining = entry.get_mut();
            *remaining -= 1;

            if *remaining == 0 {
                entry.remove();
            }
        };
    }

    occurrences.is_empty()
}
