use std::ops::Rem;

pub struct Matcher<T> {
    match_fn: fn(T) -> bool,
    replacement: &'static str,
}

impl<T> Matcher<T> {
    pub fn new(matcher: fn(T) -> bool, replacement: &'static str) -> Self {
        Self {
            match_fn: matcher,
            replacement,
        }
    }

    fn matches(&self, value: T) -> Option<&'static str> {
        (self.match_fn)(value).then_some(self.replacement)
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
#[derive(Default)]
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T: ToString + Copy> Fizzy<T> {
    pub fn new() -> Self {
        Self {
            matchers: Vec::new(),
        }
    }

    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply(self, iter: impl Iterator<Item = T>) -> impl Iterator<Item = String> {
        iter.map(move |value| {
            let result: String = self
                .matchers
                .iter()
                .filter_map(|matcher| matcher.matches(value))
                .collect();

            if result.is_empty() {
                value.to_string()
            } else {
                result
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: PartialEq + ToString + Copy + Rem<Output = T> + From<u8>,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|val: T| val % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|val: T| val % 5.into() == 0.into(), "buzz"))
}
