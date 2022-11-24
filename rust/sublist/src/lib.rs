use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match first_list.len().cmp(&second_list.len()) {
        Ordering::Less => contains(second_list, first_list).then_some(Comparison::Sublist),
        Ordering::Equal => (first_list == second_list).then_some(Comparison::Equal),
        Ordering::Greater => contains(first_list, second_list).then_some(Comparison::Superlist),
    }
    .unwrap_or(Comparison::Unequal)
}

fn contains<T: PartialEq>(bigger_list: &[T], smaller_list: &[T]) -> bool {
    smaller_list.is_empty()
        || bigger_list
            .windows(smaller_list.len())
            .any(|sublist| sublist == smaller_list)
}
