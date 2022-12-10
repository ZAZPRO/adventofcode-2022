use nom::character::complete::newline;
use nom::{branch::alt, multi::separated_list1};
use std::error::Error;

use nom::{bytes::complete::tag, sequence::preceded, IResult};

// How long each instruction takes.
pub const NOOP_CYCLES: u32 = 1;
pub const ADDX_CYCLES: u32 = 2;

// Instruction that will be executed.
#[derive(Debug, Clone, Copy)]
enum Op {
    Addx(i32),
    Noop,
}

// CPU that will execute Opcodes.
#[derive(Debug)]
struct CPU {
    // Register X that we will modify.
    x: i32,
    // Amount of ticks that has passed.
    ticks: u32,
    // Current Instruction that is executed.
    current_op: Op,
    // Amount of ticks that current instruction taks.
    current_ticks: u32,
    // List of all instructions that will be executed.
    program: Vec<Op>,
    // Is CPU ready to fetch next instruction.
    ready_to_fetch: bool,
}

// CPU initialization.
impl Default for CPU {
    fn default() -> Self {
        Self {
            x: 1,
            ticks: 1,
            current_op: Op::Noop,
            current_ticks: 0,
            program: Vec::new(),
            ready_to_fetch: true,
        }
    }
}

// CPU methods.
impl CPU {
    // Execute code.
    fn execute(&mut self) -> Vec<(u32, i32)> {
        // List to store execution results.
        let mut res: Vec<(u32, i32)> = Vec::new();

        // CPU loop
        loop {
            // If CPU is ready to fetch new instruction.
            if self.ready_to_fetch {
                match self.fetch_opcode() {
                    // Set new instruction as current.
                    Some(o) => self.current_op = o,
                    // Else -> we run out of instructions to do. Program is completed.
                    None => return res,
                }
            }

            // Execute one CPU cycle.
            self.cycle();
            // Increment total ticks executed.
            self.ticks += 1;

            // Instead of having a lot of if. I have decided to check if current tick is an interesting value.
            if vec![20, 60, 100, 140, 180, 220].contains(&self.ticks) {
                // Safe current tick and register value.
                res.push((self.ticks, self.x));
            }
        }
    }

    // Execute one CPU cycle.
    fn cycle(&mut self) {
        // Increment current ticks that is currently executed.
        self.current_ticks += 1;
        // Match current operation with current ticks that are executed.
        // If required amount of current ticks has passed -> executed instruction & reset current ticks.
        // Else block cpu fetching.
        match (self.current_op, self.current_ticks) {
            (Op::Noop, u32::MIN..=u32::MAX) => self.execute_noop(),
            (Op::Addx(_), 0..=1) => self.ready_to_fetch = false,
            (Op::Addx(v), ADDX_CYCLES..=u32::MAX) => self.execute_addx(v),
        }
    }

    // Execute Noop instructon.
    fn execute_noop(&mut self) {
        // Reset current ticks executed.
        self.current_ticks = 0;
        // Unblock CPU fetching.
        self.ready_to_fetch = true;
    }

    // Execute Addx instructon.
    fn execute_addx(&mut self, value: i32) {
        // Modify X register.
        self.x += value;
        // Prepare CPU for next instruction.
        self.execute_noop();
    }

    // Debug printing function.
    fn _debug_print(&self) {
        println!(
            "Cycle: {}, Current Op: {:?}, X: {}",
            self.ticks, self.current_op, self.x
        );
    }

    // Fetch next opcode.
    fn fetch_opcode(&mut self) -> Option<Op> {
        self.program.pop()
    }
}

// Nom addx parser.
fn parse_addx(i: &str) -> IResult<&str, Op> {
    let (i, res) = preceded(tag("addx "), nom::character::complete::i32)(i)?;

    Ok((i, Op::Addx(res)))
}

// Nom noop parser.
fn parse_noop(i: &str) -> IResult<&str, Op> {
    let (i, _) = tag("noop")(i)?;

    Ok((i, Op::Noop))
}

// Nom opcode parser.
fn parse_opcode(i: &str) -> IResult<&str, Op> {
    alt((parse_addx, parse_noop))(i)
}

// Nom file parser.
fn parse_file(i: &str) -> IResult<&str, Vec<Op>> {
    separated_list1(newline, parse_opcode)(i)
}

// Program entry point.
fn main() -> Result<(), Box<dyn Error>> {
    // Pair of CPU cycle and X register that we will use to calculate final score.
    let result: Vec<(u32, i32)>;
    // Read input file into String.
    let file = std::fs::read_to_string("../input.txt")?;
    // Get List of instructions that will be executed.
    let (_, mut res) = parse_file(file.as_str()).unwrap();
    // Reverse the order as we are interested in first instruction.
    res.reverse();

    // Initiate CPU with default values.
    let mut cpu = CPU::default();
    // Set it's current program to the one we just parsed from a file.
    cpu.program = res;

    // Execute CPU program and store execution result.
    result = cpu.execute();

    // Variable to calculate final sum.
    let mut sum: i32 = 0;

    // For each stored execution result.
    for res in result {
        // Multiply cycles by X register value.
        sum += res.0 as i32 * res.1;
    }

    // Print the result.
    println!("Final sum: {sum}");

    Ok(())
}
