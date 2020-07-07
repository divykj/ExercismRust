use std::collections::{BTreeMap, BTreeSet};

pub struct School {
    grades: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let students = self.grades.entry(grade).or_insert(BTreeSet::new());
        students.insert(String::from(student));
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().cloned().collect()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        match self.grades.get(&grade) {
            Some(students) => Some(students.iter().cloned().collect()),
            _ => None,
        }
    }
}
