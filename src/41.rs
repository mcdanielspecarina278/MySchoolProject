// Assume this is part of your MySchoolProject project.
use std::collections::HashMap;

#[derive(Debug)]
struct Student {
    name: String,
    grade: i32,
}

fn main() {
    let students = vec![
        Student {
            name: "John".to_string(),
            grade: 10,
        },
        Student {
            name: "Jane".to_string(),
            grade: 11,
        },
        Student {
            name: "Tom".to_string(),
            grade: 12,
        },
    ];

    println!("Name, Grade");
    for student in students {
        println!("{}, {}", student.name, student.grade);
    }
}
