use std::collections::HashSet;
use std::error::Error;

// Main program entty.
fn main() -> Result<(), Box<dyn Error>> {
    // Total result sum.
    let mut total_sum: u32 = 0;

    // Read input file into String.
    let file = std::fs::read_to_string("../input.txt")?;
    let lines: Vec<&str> = file.lines().collect();
    // For each new line.
    for chunk in lines.chunks(3) {
        // Transform strs into slices of bytes.
        let first_member = chunk[0].as_bytes();
        let second_member = chunk[1].as_bytes();
        let third_member = chunk[2].as_bytes();

        // Create a Set out of the first member.
        let mut a: HashSet<u8> = HashSet::new();
        for v in first_member {
            a.insert(*v);
        }
        // Create a Set out of the second member.
        let mut b: HashSet<u8> = HashSet::new();
        for v in second_member {
            b.insert(*v);
        }

        // Create a Set out of the third member.
        let mut c: HashSet<u8> = HashSet::new();
        for v in third_member {
            c.insert(*v);
        }

        // B&C intersection vector.
        let b_c_intersection: Vec<&u8> = b.intersection(&c).collect();
        // Create a Set out of the second and third intersection.
        let mut bc: HashSet<u8> = HashSet::new();
        for v in b_c_intersection {
            bc.insert(*v);
        }

        // Find intersection of three sets, that's our common item.
        for v in a.intersection(&bc) {
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
