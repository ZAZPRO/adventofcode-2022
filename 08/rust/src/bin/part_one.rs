use std::{collections::HashMap, error::Error};

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

    // Varibale to store total amout of visible trees.
    let mut visible_trees: usize = 0;

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

            // Check if all values in tree lists are lower than current tree height.
            let all_trees_smaller_up: bool = trees_up.iter().all(|v| *v < *current_tree);
            let all_trees_smaller_down: bool = trees_down.iter().all(|v| *v < *current_tree);
            let all_trees_smaller_left: bool = trees_left.iter().all(|v| *v < *current_tree);
            let all_trees_smaller_right: bool = trees_right.iter().all(|v| *v < *current_tree);

            // If tree is visible from any direction -> increase counter.
            if all_trees_smaller_up
                || all_trees_smaller_down
                || all_trees_smaller_left
                || all_trees_smaller_right
            {
                visible_trees += 1;
            }
        }
    }

    // Also calculate all outer trees as they are visible by default.
    visible_trees += x_max + (x_max - 2) + (y_max * 2 - 2);
    // Print the result!
    println!("Amout of visible trees: {}", visible_trees);

    Ok(())
}
