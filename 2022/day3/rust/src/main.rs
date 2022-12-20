use std::fs;

fn main() {
    // Read the list of rucksacks from input.txt
    let rucksacks = read_rucksacks();

    // Find the common item type for each rucksack
    let common_item_types = rucksacks.iter()
        .map(|r| find_common_item_type(r))
        .collect::<Vec<char>>();

    // Calculate the priority for each item type
    let priorities = common_item_types.iter()
        .map(|t| calculate_priority(*t))
        .collect::<Vec<u32>>();

    // Calculate the sum of the priorities
    let sum: u32 = priorities.iter().sum();

    // Print the result
    println!("The sum of the priorities is {}", sum);
}

fn read_rucksacks() -> Vec<String> {
    // Read the data from input.txt
    let data = fs::read_to_string("/Users/jacobkarlstrom/projekt/advent_of_code/day3/input.txt").expect("Failed to read input.txt");


    // Parse the data into a vector of rucksacks
    data.lines().map(|line| line.to_string()).collect()
}

fn find_common_item_type(rucksack: &str) -> char {
    // Find common chars from two vectors of chars
    let common_chars = rucksack.chars()
        .filter(|c| rucksack.chars().filter(|d| d == c).count() > 1)
        .collect::<Vec<char>>();
        

    
}

fn calculate_priority(item_type: char) -> u32 {
    // TODO: Calculate the priority for the given item type

}
