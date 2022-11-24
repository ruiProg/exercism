use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub struct School<'a> {
    student_grafes: BTreeMap<u32, BTreeSet<&'a str>>,
}

impl<'a> School<'a> {
    pub fn new() -> School<'a> {
        Self::default()
    }

    pub fn add(&mut self, grade: u32, student: &'a str) {
        let students = self.student_grafes.entry(grade).or_default();
        students.insert(student);
    }

    pub fn grades(&self) -> Vec<u32> {
        self.student_grafes.keys().copied().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.student_grafes
            .get(&grade)
            .unwrap_or(&BTreeSet::new())
            .iter()
            .map(|student| student.to_string())
            .collect()
    }
}
