// Define the struct for a student
struct Student {
    name: String,
    age: u8,
}

// Function to add students to a list
fn add_students(students: &mut Vec<Student>, new_student: &Student) {
    // Add the new student to the list
    students.push(new_student);
}

// Main function for managing students
fn main() {
    let mut students = vec![];

    // Example data: 5 students with their names and ages
    let students_data = vec![
        ("Alice", 13),
        ("Bob", 12),
        ("Charlie", 14),
        ("David", 16),
        ("Eve", 10),
    ];

    for (name, age) in &students_data {
        // Add each student to the list
        add_students(&mut students, &Student { name: name.to_string(), age });
    }

    // Print the names of the students
    for (i, student) in students.iter().enumerate() {
        println!("Student {} - {}", i + 1, student.name);
    }
}
