use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Read input file into String.
    let file = std::fs::read_to_string("../input.txt")?;
    // Collect all file characters into a vector.
    let c: Vec<char> = file.trim_end().chars().collect();

    // Get sliding window of those characters.
    let sliding_window = c.windows(14);
    // Enumerate those.
    'outer: for (i, window) in sliding_window.enumerate() {
        // For each other value.
        for k in 0..14 {
            for j in (1 + k)..14 {
                // Check if they are the same and go to the next window if they are.
                if window[k] == window[j] {
                    continue 'outer;
                }
            }
        }

        // We end up here only if all numbers are different in a window.
        println!("{}: {:?}", i + 14, window);
        break;
    }

    Ok(())
}
