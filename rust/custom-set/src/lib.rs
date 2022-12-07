#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T> {
    data: Vec<T>,
}

impl<T: Copy + Ord> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut set = Self {
            data: Vec::with_capacity(input.len()),
        };
        input.iter().for_each(|&element| set.add(element));

        set
    }

    pub fn contains(&self, element: &T) -> bool {
        self.data.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if let Some(pos) = self.data.iter().position(|&value| value >= element) {
            if self.data[pos] != element {
                self.data.insert(pos, element);
            }
        } else {
            self.data.push(element)
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.data.iter().all(|element| other.contains(element))
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.data.iter().all(|element| !other.contains(element))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let items = self
            .data
            .iter()
            .filter(|element| other.contains(element))
            .copied()
            .collect();
        Self { data: items }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let items = self
            .data
            .iter()
            .filter(|element| !other.contains(element))
            .copied()
            .collect();
        Self { data: items }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut set = Self::new(&self.data);
        other.data.iter().for_each(|&element| set.add(element));
        set
    }
}
