use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    list.first().map_or(String::new(), |first_item| {
        list.windows(2)
            .map(|items| format!("For want of a {} the {} was lost.\n", items[0], items[1]))
            .chain(once(format!("And all for the want of a {first_item}.")))
            .collect()
    })
}
