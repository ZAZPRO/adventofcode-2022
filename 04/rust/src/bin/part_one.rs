use std::collections::HashSet;
use std::error::Error;

// Main program entty.
fn main() -> Result<(), Box<dyn Error>> {
    // Total result sum.
    let mut total_sum: u32 = 0;

    // Read input file into String.
    let file = std::fs::read_to_string("../input.txt")?;
    // For each new line.
    for line in file.lines() {
        // Split puzzle input into two parts separated by comma.
        let (first_half, second_half) = line.rsplit_once(',').unwrap();
        // Split each part into start and end indecies.
        let (first_section_start_string, first_section_end_string) =
            first_half.rsplit_once('-').unwrap();
        let (second_section_start_string, second_section_end_string) =
            second_half.rsplit_once('-').unwrap();

        // Parse those indecies into numbers.
        let first_section_start: u32 = first_section_start_string.parse().unwrap();
        let first_section_end: u32 = first_section_end_string.parse().unwrap();
        let second_section_start: u32 = second_section_start_string.parse().unwrap();
        let second_section_end: u32 = second_section_end_string.parse().unwrap();

        // Create a Set out of the first section range.
        let mut a: HashSet<u32> = HashSet::new();
        for v in first_section_start..first_section_end + 1 {
            a.insert(v);
        }

        // Create a Set out of the second section range.
        let mut b: HashSet<u32> = HashSet::new();
        for v in second_section_start..second_section_end + 1 {
            b.insert(v);
        }

        // If first section range is subset or superset of second range, that means that we have overlap.
        if a.is_subset(&b) || a.is_superset(&b) {
            total_sum += 1;
        }
    }

    // Print the result!
    println!("Total Sum: {total_sum}");
    Ok(())
}
