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
        // Transform String to a slice of bytes.
        let bytes = line.as_bytes();
        // Split it by half in two slices.
        let (first_half, second_half) = bytes.split_at(bytes.len() / 2);
        // Create a Set out of the first half.
        let mut a: HashSet<u8> = HashSet::new();
        for v in first_half {
            a.insert(*v);
        }
        // Create a Set out of the second half.
        let mut b: HashSet<u8> = HashSet::new();
        for v in second_half {
            b.insert(*v);
        }

        // Find intersection of two sets, that's our common item.
        for v in a.intersection(&b) {
            // Calculate priority for each item and add it to the total sum.
            if *v > 96 {
                total_sum += u32::from(*v) - 96;
            } else {
                total_sum += u32::from(*v) - 38;
            }
        }
    }

    // Print the result!
    println!("Total Sum: {total_sum}");
    Ok(())
}
