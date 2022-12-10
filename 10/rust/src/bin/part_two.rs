use nom::character::complete::newline;
use nom::{branch::alt, multi::separated_list1};
use nom::{bytes::complete::tag, sequence::preceded, IResult};
use std::error::Error;

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
            current_op: Op::Noop,
            current_ticks: 0,
            program: Vec::new(),
            ready_to_fetch: true,
        }
    }
}

// CPU methods.
impl CPU {
    // Execute one CPU cycle.
    fn execute_one_cycle(&mut self) -> Option<()> {
        // If CPU is ready to fetch new instruction.
        if self.ready_to_fetch {
            match self.fetch_opcode() {
                // Set new instruction as current.
                Some(o) => self.current_op = o,
                // Else -> we run out of instructions to do. Program is completed.
                None => return None,
            }
        }

        // One CPU cycle.
        self.cycle();

        // Return success.
        Some(())
    }

    // Actually do one cycle.
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

    // Fetch next opcode.
    fn fetch_opcode(&mut self) -> Option<Op> {
        self.program.pop()
    }
}

#[derive(Default)]
struct CRT {
    x: u32,
}

impl CRT {
    // Execute one CRT cycle.
    fn execute_one_cycle(&mut self, reg_x: i32, ticks: u32) {
        // Actually do one cycle.
        self.cycle(reg_x, ticks);
        // Increment x counter.
        self.x += 1;
        // Used to have more logic there, that's why this method exists...
    }

    // Actually do one cycle.
    fn cycle(&mut self, reg_x: i32, ticks: u32) {
        //  If tick is a multiple of 40 (and greater that 1).
        if (ticks > 1) && (((ticks - 1) % 40) == 0) {
            // Print new line.
            println!();
        }

        // If we are in the end of the row -> reset x counter.
        if self.x > 39 {
            self.x = 0;
        }

        // If CPU register X has value x +- 1 relative to current crt x counter.
        if (((reg_x - 1) as u32) <= self.x) && (((reg_x + 1) as u32) >= self.x) {
            // It is a sprite.
            print!("#");
        } else {
            // Else it's nothing.
            print!(".");
        }
    }
}

// Struct that holds Personal Computer which consists out of CPU & CRT.
struct PC {
    cpu: CPU,
    crt: CRT,
    ticks: u32,
}

impl PC {
    // PC constructor.
    fn new(cpu: CPU, crt: CRT, ticks: u32) -> Self {
        Self { cpu, crt, ticks }
    }

    // Execute CPU program & render CRT.
    fn execute(&mut self) {
        loop {
            // If CPU is ready to execute next instruction.
            if self.cpu.ready_to_fetch {
                // Execute it.
                let res = self.cpu.execute_one_cycle();
                // If there are no instructions left -> return.
                if res.is_none() {
                    break;
                }
                // Render CRT.
                self.crt.execute_one_cycle(self.cpu.x, self.ticks);
            } else {
                // CPU is busy -> render CRC first.
                self.crt.execute_one_cycle(self.cpu.x, self.ticks);
                // Then execute CPU cycle.
                let res = self.cpu.execute_one_cycle();
                // If there are no instructions left -> return.
                if res.is_none() {
                    break;
                }
            }
            // Increment cycle counter.
            self.ticks += 1;
        }
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
    // Initialize CRT with default values.
    let crt = CRT::default();

    // Iinitialize PC with CPU & CRT that we have just created.
    let mut pc = PC::new(cpu, crt, 1);
    // Execute CPU program.
    pc.execute();

    Ok(())
}
