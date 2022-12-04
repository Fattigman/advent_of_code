use std::fs;
use std::collections::HashMap;


fn main() {
    // Specify the path to the file you want to read
    let file_path = "/Users/jacobkarlstrom/projekt/advent_of_code/day2/input.txt";

    // Read the contents of the file into a string
    let file_contents = fs::read_to_string(file_path)
        .expect("Failed to read file");
    let opponent_scores = [
        ('A', "Rock"),
        ('B', "Paper"),
        ('C', "Scissors"),
        ].iter().cloned().collect::<HashMap<_, _>>();
        
    let you_scores = [
        ('X', "Rock"),
        ('Y', "Paper"),
        ('Z', "Scissors"),
         ].iter().cloned().collect::<HashMap<_, _>>();

    // Initialize the total score to 0
    let mut total_score: i32 = 0;
    // Loop through the strategy guide and calculate the score for each round
    for line in file_contents.lines() {
        // Parse the opponent"s shape and your shape from the line
        let opponent = line.chars().nth(0).unwrap();
        let you = line.chars().nth(2).unwrap();
        // Assign bonus points for the round based on your shape
        if you_scores[&you] == "Rock" {
            total_score += 1;
        }
        else if you_scores[&you] == "Paper" {
            total_score += 2;
        }
        else if you_scores[&you] == "Scissors" {
            total_score += 3;
        }
        // Define the winning shapes
        let win_condition = match ( you_scores[&you], opponent_scores[&opponent]) {
            ("Rock", "Scissors") => true,
            ("Paper", "Rock") => true,
            ("Scissors", "Paper") => true,
            _ => false,
            };
        
        // Check if the round is a draw
        if opponent_scores[&opponent] == you_scores[&you] {
            total_score += 3;
        } else if win_condition  {
            total_score += 6;
        } else {
        }
    }
println!("Total score: {}", total_score);

}
