use std::error::Error;

// Really wanted to learn this crate, now I have an excuse :)
extern crate nom;
use nom::{
    branch::alt,
    bytes::complete::is_a,
    bytes::complete::tag,
    character::complete::{char, newline},
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};

// Struct to hold one move action.
struct Move {
    amount: usize,
    from_index: usize,
    to_index: usize,
}

// Nom parser that matches [X] where X is an uppercase letter.
fn crates_parser(input: &str) -> IResult<&str, char> {
    let (input, c) = preceded(
        char('['),
        terminated(is_a("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), char(']')),
    )(input)?;

    Ok((input, c.chars().next().unwrap()))
}

// Nom parser that matches "   " that means it's an empty cargo there.
fn empty_cargo_parser(input: &str) -> IResult<&str, char> {
    let (input, _) = tag("   ")(input)?;

    Ok((input, ' '))
}

// Nom parser to parse just one line of setup string.
fn setup_line_parser(input: &str) -> IResult<&str, Vec<char>> {
    separated_list1(tag(" "), alt((crates_parser, empty_cargo_parser)))(input)
}

// Nom parser to parse multiple lines of setup string.
fn setup_lines_parser(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(newline, setup_line_parser)(input)
}

// Nom parser to parse last line of setup string.
fn crate_numbers_line_parser(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag("   "), nom::character::complete::u32)(input)
}

// Nom parser that parses line of move string.
fn move_line_parser(input: &str) -> IResult<&str, Move> {
    let (input, amount) = preceded(tag("move "), nom::character::complete::u32)(input)?;
    let (input, from_index) = preceded(tag(" from "), nom::character::complete::u32)(input)?;
    let (input, to_index) = preceded(tag(" to "), nom::character::complete::u32)(input)?;

    Ok((
        input,
        Move {
            amount: amount.try_into().unwrap(),
            from_index: from_index.try_into().unwrap(),
            to_index: to_index.try_into().unwrap(),
        },
    ))
}

// Nom parser that parses multiple lines of move string.
fn move_lines_parser(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list1(newline, move_line_parser)(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Reading and parsing file into starting situation and moves we have to do.
    let file = std::fs::read_to_string("../input.txt")?;
    let mut setup_string = String::new();
    let mut moves_string = String::new();

    let mut append_to_setup = true;
    for line in file.lines() {
        if line.is_empty() {
            append_to_setup = false;
            continue;
        }

        if append_to_setup {
            setup_string.push_str(line);
            setup_string.push('\n');
        } else {
            moves_string.push_str(line);
            moves_string.push('\n');
        }
    }

    // Use Nom to parse setup crates positions into char vector.
    let (left, crates) = setup_lines_parser(setup_string.as_str()).unwrap();
    // Use Nom to parse last line of initial crates positions into a vector of crates numbers.
    let (_, mut crate_numbers) = crate_numbers_line_parser(left.trim()).unwrap();
    // Use crate_numbers vector to get total crates number.
    let total_crates_number: usize = crate_numbers.pop().unwrap().try_into().unwrap();

    // Initialization of list that holds all each stacks of crates.
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(total_crates_number);
    for _ in 0..total_crates_number {
        stacks.push(Vec::new())
    }

    // Go through each parsed crates list and append it into our stacks vector.
    crates.iter().rev().for_each(|r| {
        r.iter().enumerate().for_each(|v| {
            if *v.1 != ' ' {
                stacks.get_mut(v.0).unwrap().push(*v.1);
            }
        })
    });

    // Use nom to get parsed list of Moves.
    let (_, moves) = move_lines_parser(moves_string.as_str()).unwrap();
    // For each move in parsed moves.
    for mov in moves {
        // Get amount of moves we have to do. And for each of them.
        for _ in 0..mov.amount {
            // Pop a top value from required stack.
            let v = stacks.get_mut(mov.from_index - 1).unwrap().pop().unwrap();
            // And push it into anouther required stack.
            stacks.get_mut(mov.to_index - 1).unwrap().push(v);
        }
    }

    // Pop and print top values from all stacks to get final result.
    for mut stack in stacks {
        let value = stack.pop().unwrap();
        print!("{value}");
    }

    Ok(())
}
