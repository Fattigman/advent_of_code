use std::fs;

fn main() {
    // Specify the path to the file you want to read
    let file_path = "/Users/jacobkarlstrom/projekt/advent_of_code/day1/input.txt";

    // Read the contents of the file into a string
    let file_contents = fs::read_to_string(file_path)
        .expect("Failed to read file");
    
    let mut highest: i32 = 0;
    let mut current: i32 = 0;
    // Loop through each line in the file
    for line in file_contents.lines() {
        if line != "" {
            let num: i32 = line.parse().unwrap();
            current += num;
            if current > highest {
                highest = current;
            }
        }
        else {
            if current > highest {
                highest = current;
            }
            current = 0;
        }
    }
    println!("Highest calories: {}", highest);
}
