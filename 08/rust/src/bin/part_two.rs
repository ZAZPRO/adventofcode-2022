use std::{collections::HashMap, error::Error};

// Function that will calculate trees scene score.
fn calculate_tree_scene_score(current_tree_height: u32, trees_on_its_way: Vec<u32>) -> usize {
    let mut scenic_score: usize = 0;
    // For each tree in the list
    for tree in trees_on_its_way {
        // Add a score point.
        scenic_score += 1;
        // If tree bloks view -> we have done calculating.
        if tree >= current_tree_height {
            break;
        }
    }
    scenic_score
}

fn main() -> Result<(), Box<dyn Error>> {
    // HashMap that will hold a map of trees.
    let mut trees: HashMap<(usize, usize), u32> = HashMap::new();

    // Read input file into String.
    let file = std::fs::read_to_string("../input.txt")?;
    // Amount of lines is a total of y coordinates.
    let y_max = file.lines().count();
    // Amount of chars in fist line is a total of x coordinates.
    let x_max = file.lines().next().unwrap().chars().count();

    // For each new line inside of that string.
    for (y, line) in file.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            // Treat each character as x and each line as y.
            // Insert a three height into the map at those coordinates.
            trees.insert((x, y), c.to_digit(10).unwrap());
        }
    }

    // List to store scenic scores of all trees.
    let mut scenic_scores: Vec<usize> = Vec::new();

    // For each inside tree.
    for y in 1..y_max - 1 {
        for x in 1..x_max - 1 {
            // Get current tree height
            let current_tree = trees.get(&(x, y)).unwrap();

            // Get a List of all trees in all directions.
            let mut trees_up: Vec<u32> = Vec::new();
            for k in 1..y + 1 {
                trees_up.push(*trees.get(&(x, y - k)).unwrap());
            }

            let mut trees_down: Vec<u32> = Vec::new();
            for k in 1..y_max - y {
                trees_down.push(*trees.get(&(x, y + k)).unwrap());
            }

            let mut trees_left: Vec<u32> = Vec::new();
            for k in 1..x + 1 {
                trees_left.push(*trees.get(&(x - k, y)).unwrap());
            }

            let mut trees_right: Vec<u32> = Vec::new();
            for k in 1..x_max - x {
                trees_right.push(*trees.get(&(x + k, y)).unwrap());
            }

            // Calculate tree scene score for every direction.
            let scenic_score_up: usize = calculate_tree_scene_score(*current_tree, trees_up);
            let scenic_score_down: usize = calculate_tree_scene_score(*current_tree, trees_down);
            let scenic_score_left: usize = calculate_tree_scene_score(*current_tree, trees_left);
            let scenic_score_right: usize = calculate_tree_scene_score(*current_tree, trees_right);

            // Push scenic score to a list of scores.
            scenic_scores
                .push(scenic_score_up * scenic_score_down * scenic_score_left * scenic_score_right);
        }
    }

    // Print the result!
    println!(
        "Biggest scene score: {}",
        scenic_scores.iter().max().unwrap()
    );

    Ok(())
}
