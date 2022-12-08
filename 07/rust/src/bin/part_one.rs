// This solution is meh. It's 1 AM of the next day for me already... Should have done more iterators...

// As I want to get better with Nom I will keep using it.
use nom::{
    branch::alt,
    bytes::complete::is_a,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::preceded,
    sequence::separated_pair,
    IResult,
};
use std::{collections::HashMap, error::Error};

// Enum to hold Terminal Output. Nom parsers will output it.
enum TerminalOutput {
    LFile(u64),
    LDir(String),
    Cd(String),
    Ls,
}

// Different Nom parsers bellow. If interested check other solutions I have.
fn parse_cd(i: &str) -> IResult<&str, TerminalOutput> {
    let (i, res) = preceded(tag("$ cd "), alt((alpha1, is_a("../"))))(i)?;

    Ok((i, TerminalOutput::Cd(res.to_string())))
}

fn parse_ls(i: &str) -> IResult<&str, TerminalOutput> {
    let (i, _res) = tag("$ ls")(i)?;

    Ok((i, TerminalOutput::Ls))
}

fn parse_command(i: &str) -> IResult<&str, TerminalOutput> {
    let (i, res) = alt((parse_cd, parse_ls))(i)?;

    Ok((i, res))
}

fn parse_lfile(i: &str) -> IResult<&str, TerminalOutput> {
    let (i, res) = separated_pair(
        nom::character::complete::u64,
        tag(" "),
        is_a("abcdefghijklmnopqrstuvwxyz."),
    )(i)?;

    Ok((i, TerminalOutput::LFile(res.0)))
}

fn parse_ldir(i: &str) -> IResult<&str, TerminalOutput> {
    let (i, res) = preceded(tag("dir "), alpha1)(i)?;

    Ok((i, TerminalOutput::LDir(res.to_string())))
}

fn parse_listed(i: &str) -> IResult<&str, TerminalOutput> {
    let (i, res) = alt((parse_lfile, parse_ldir))(i)?;

    Ok((i, res))
}

fn parse_line(i: &str) -> IResult<&str, TerminalOutput> {
    let (i, res) = alt((parse_command, parse_listed))(i)?;

    Ok((i, res))
}

fn parse_lines(i: &str) -> IResult<&str, Vec<TerminalOutput>> {
    separated_list1(newline, parse_line)(i)
}

// Struct to hold a directory.
pub struct MyDir {
    pub size: u64,                    // Size of directory.
    pub files: Vec<u64>,              // List of file sizes. (As we don't need names.)
    pub dirs: HashMap<String, MyDir>, // HashMap with the name and nested directory itself.
}

// For this struct.
impl MyDir {
    // Implement new function, that initializes it. (Should have used Default for this usecase.)
    fn new() -> Self {
        Self {
            size: 0,
            files: Vec::new(),
            dirs: HashMap::new(),
        }
    }

    // Returns a sum of file sizes that are stored in this directory.
    fn get_files_size(&self) -> u64 {
        self.files.iter().sum()
    }

    // Travels through each nested directory and calculates it's size.
    fn calc_total_size(&mut self) -> u64 {
        // Get size of files inside this directory.
        let mut total_sum = self.get_files_size();

        // Get all directories that are nested inside of this directory.
        let dirs: Vec<&mut MyDir> = self.dirs.values_mut().collect();
        // For each one of them.
        for dir in dirs {
            // Recursively call this function and add this to a current total sum.
            total_sum += dir.calc_total_size();
        }

        // Safe calculated sum.
        self.size = total_sum;
        // Return it.
        total_sum
    }

    // Returns a list of nested directories sizes.
    fn get_total_sizes(&mut self) -> Vec<u64> {
        // Create a vector and push this directory size into it.
        let mut sizes: Vec<u64> = Vec::new();
        sizes.push(self.size);

        // Get all directories that are nested inside of this directory.
        let dirs: Vec<&mut MyDir> = self.dirs.values_mut().collect();
        // For each one of them.
        for dir in dirs {
            // Recursively call this function and append this to a current vector of sizes.
            sizes.append(&mut dir.get_total_sizes());
        }

        // Return this vector.
        sizes
    }
}

// Main program entry.
fn main() -> Result<(), Box<dyn Error>> {
    // Read input file into String.
    let file = std::fs::read_to_string("../input.txt")?;
    // Parse file into a List of TerminalOutput enums.
    let (_, res) = parse_lines(file.as_str()).unwrap();

    // Initialize root directory.
    let mut root = MyDir::new();
    root.dirs.insert("/".to_string(), MyDir::new());

    // This a Stack that will hold current path that we are in.
    let mut current_path: Vec<String> = Vec::new();

    // For each terminal output, match it with specific enum instance.
    for output in res {
        match output {
            // If we have a 'cd' command.
            TerminalOutput::Cd(path) => {
                // Push current cd path into current_path stack, unless it's a '..' path.
                // In that case we pop a path.
                if path == ".." {
                    current_path.pop();
                } else {
                    current_path.push(path);
                }
            }
            TerminalOutput::Ls => (),
            // If we have a file entry.
            TerminalOutput::LFile(size) => {
                // Go through each directory in our path stack starting from root.
                let mut current_dir: &mut MyDir = &mut root;
                for path in &current_path {
                    current_dir = current_dir.dirs.get_mut(path).unwrap();
                }

                // Last directory we went through is the one where we want to add new files into.
                current_dir.files.push(size);
            }
            // If we have a directory entry.
            TerminalOutput::LDir(name) => {
                // Go through each directory in our path stack starting from root.
                let mut current_dir: &mut MyDir = &mut root;
                for path in &current_path {
                    current_dir = current_dir.dirs.get_mut(path).unwrap();
                }

                // Last directory we went through is the one where we want to add new directory into.
                current_dir.dirs.insert(name, MyDir::new());
            }
        }
    }

    // At this point we have finished creating a file tree.
    // Now we can go and calculate sizes of all directories inside of it.
    root.calc_total_size();

    // And get a List of all sizes to filter those to the challange requirement.
    let total_sizes: Vec<u64> = root
        .get_total_sizes()
        .into_iter()
        .filter(|x| *x <= 100000)
        .collect();

    // Calculate final sum and print output.
    println!("Final sum: {}", total_sizes.into_iter().sum::<u64>());

    // Successfully return.
    Ok(())
}
