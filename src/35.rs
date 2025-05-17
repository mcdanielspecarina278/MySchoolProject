use std::fs;

fn main() {
    let file_path = "data.csv"; // Replace this with your actual data file path

    // Read the CSV file into a vector of vectors containing each row as a separate element
    let rows: Vec<Vec<&str>> = fs::read_to_string(file_path).unwrap().split('\n').map(|line| line.split(',')).collect();

    // Sort the rows by the first column in alphabetical order
    rows.sort_by_key(|row| row[0]);

    // Print the sorted CSV file to stdout
    for row in &rows {
        println!("{:?}", row);
    }
}
