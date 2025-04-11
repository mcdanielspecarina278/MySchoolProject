use std::collections::HashMap;

// Define the data structures and constants here

fn main() {
    let student_data = vec![
        (1, "John"),
        (2, "Alice"),
        (3, "Bob"),
        // Add more students as needed
    ];

    for (index, &student) in student_data.iter().enumerate() {
        println!("Student {}: {}", index + 1, student);
    }
}
