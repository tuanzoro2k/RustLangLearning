use std::collections::HashMap;
pub struct School<T> {
    students: HashMap<String, T>,
}

impl<T> School<T>
where
    T: Clone + Ord,
{
    pub fn new() -> Self {
        Self {
            students: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: T, student: &str) {
        self.students.insert(student.to_string(), grade);
    }

    pub fn grades(&self) -> Vec<T> {
        let mut _grades: Vec<T> = self.students.values().cloned().collect();
        _grades.sort();
        _grades.dedup();
        _grades
    }

    pub fn grade(&self, grade: T) -> Vec<String> {
        let mut result = Vec::<String>::new();
        for (_student, _grade) in &self.students {
            if grade == *_grade {
                result.push(_student.to_string());
            }
        }
        result.sort();
        result
    }
}
fn main() {
    let mut student1 = School::new();
    student1.add("D".to_string(), "Tuan");
    student1.add("A".to_string(), "Duc");
    student1.add("B".to_string(), "Dung");
    student1.add("A".to_string(), "Alice");
    let grades = student1.grades();
    println!("{:?}", grades);
    let grade = student1.grade("A".to_string());
    println!("{:?}", grade);
}
