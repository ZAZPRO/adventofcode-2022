use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Read input file into String.
    let file = std::fs::read_to_string("../input.txt")?;
    // Collect all file characters into a vector.
    let c: Vec<char> = file.trim_end().chars().collect();

    // Get sliding window of those characters.
    let sliding_window = c.windows(4);
    // Enumerate those.
    for (i, window) in sliding_window.enumerate() {
        // If any character matches any other character.
        if window[0] != window[1]
            && window[0] != window[2]
            && window[0] != window[3]
            && window[1] != window[2]
            && window[1] != window[3]
            && window[2] != window[3]
        {
            // We have found sequence! Print & Return.
            println!("{}: {:?}", i + 4, window);
            return Ok(());
        }
    }

    Ok(())
}
