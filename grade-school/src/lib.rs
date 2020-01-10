use std::collections::HashMap;

pub struct School {
    rooster: HashMap<u32, Vec<String>>
}

impl School {
    pub fn new() -> School {
        Self { rooster: HashMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if let Some(students) = self.rooster.get_mut(&grade) {
            (*students).push(String::from(student))
        } else {
            self.rooster.insert(grade, vec![String::from(student)]);
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = self.rooster.keys().cloned().collect();
        grades.sort();
        grades
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.rooster.get(&grade).cloned().map(|mut students| {
            students.sort();
            students
        })
    }
}
