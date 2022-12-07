use std::cmp::Ordering;

pub fn find<T: Ord>(container: impl AsRef<[T]>, key: T) -> Option<usize> {
    let mut slice = container.as_ref();
    let mut lower_bound = 0;

    while !slice.is_empty() {
        let mid = slice.len() / 2;
        let (smaller, bigger) = slice.split_at(mid);

        slice = match key.cmp(&slice[mid]) {
            Ordering::Equal => return Some(lower_bound + mid),
            Ordering::Less => smaller,
            Ordering::Greater => {
                lower_bound += mid + 1;
                bigger.get(1..).unwrap_or_default()
            }
        };
    }

    None
}
