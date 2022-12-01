use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // List to hold sums.
    let mut sums: Vec<u32> = Vec::new();
    // Current sum.
    let mut current_sum: u32 = 0;

    // Read input file into String.
    let file = std::fs::read_to_string("../input.txt")?;
    // For each new line inside of that string.
    for line in file.lines() {
        // If it's empty push current sum and reset it.
        if line.is_empty() {
            sums.push(current_sum);
            current_sum = 0;
        } else {
            // If we have a value, parse it and add to the list.
            current_sum += line.parse::<u32>()?;
        }
    }

    // Get max value from the list.
    let max_callories = sums.iter().max().unwrap();
    // Get index of that value.
    let elf_index = sums.iter().position(|x| x == max_callories).unwrap();

    // Print the result!
    println!(
        "Elf number: {}, got: {} callories!",
        elf_index, max_callories
    );

    Ok(())
}
