// Using feature that will be stable in just 2 weeks from 01.12.2022
#![feature(map_first_last)]
use std::collections::BTreeMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Store future sums as B-tree, as those are sorted by index.
    let mut sums: BTreeMap<u32, u32> = BTreeMap::new();
    // Variable to store current sum.
    let mut current_sum: u32 = 0;
    // Variable to store current elf index.
    let mut current_elf: u32 = 0;

    // Read file into a String.
    let file = std::fs::read_to_string("../input.txt")?;
    // For each new line.
    for line in file.lines() {
        // If line is empty, append sum and elf index to a B-tree.
        if line.is_empty() {
            sums.insert(current_sum, current_elf);
            current_sum = 0;
            current_elf += 1;
        } else {
            // Else parse a String into a number and calculate current sum.
            current_sum += line.parse::<u32>()?;
        }
    }

    // Pop last 3 values from a B-tree and calculate final sum.
    println!("Top 3 Elves!");
    let mut final_sum: u32 = 0;
    for _ in 0..3 {
        let (sum, elf) = sums.pop_last().unwrap();
        final_sum += sum;
        println!("Elf: {elf}, Callories: {sum}");
    }
    println!("Total callories for top 3 elves: {final_sum}");

    Ok(())
}
