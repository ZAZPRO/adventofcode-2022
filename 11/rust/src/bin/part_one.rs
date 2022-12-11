use nom::branch::alt;
use nom::character::complete::digit1;
use nom::character::complete::newline;
use nom::sequence::terminated;
use nom::{bytes::complete::tag, multi::separated_list1};
use std::error::Error;

use nom::{
    sequence::{delimited, preceded},
    IResult,
};

// Type that will match any operation over 2 u64's that produces u64.
type Binop = fn(u64, u64) -> u64;

// Add function. Pointer to which will be stored inside Operation struct.
fn add(lh: u64, rh: u64) -> u64 {
    lh + rh
}

// Multiply function. Pointer to which will be stored inside Operation struct.
fn mul(lh: u64, rh: u64) -> u64 {
    lh * rh
}

// Enum that might hold 2 variants. A number or and old value. These define rh operand of Binop type function.
#[derive(Debug, Copy, Clone)]
enum Operand {
    Old,
    Number(u64),
}

// Struct that will hold Operation that should be done on the number itself and it's rh operand.
#[derive(Debug, Copy, Clone)]
struct Operation {
    operation: Binop,
    operand: Operand,
}

impl Operation {
    fn new(operation: Binop, operand: Operand) -> Self {
        Self { operation, operand }
    }

    // Execute operation itself.
    fn execute(&self, old_value: u64) -> u64 {
        match self.operand {
            // If Operand enum is Old. Than we apply operation with lh and rh operands set to old_value.
            Operand::Old => (self.operation)(old_value, old_value),
            // If Operand enum is a Number. We apply operation with lh set to old_value and rh set to number stored inside of this enum.
            Operand::Number(n) => (self.operation)(old_value, n),
        }
    }
}

// Struct that will hold all of the required information about monkey.
#[derive(Debug, Clone)]
struct Monkey {
    // It's id.
    id: usize,
    // List of items worry levels it holds.
    items: Vec<u64>,
    // Operation struct that will be used to do manipulation with worry levels.
    operation: Operation,
    // Number that will be used for % branching checks.
    divisible: u64,
    // Id of monkey we pass new worry value if brancking check passes.
    id_true: usize,
    // Id of monkey we pass new worry value if brancking check fails.
    id_false: usize,
    // Amount of items monkey has inspected.
    inspected: usize,
}

impl Monkey {
    fn new(
        id: usize,
        items: Vec<u64>,
        operation: Operation,
        divisible: u64,
        id_true: usize,
        id_false: usize,
        inspected: usize,
    ) -> Self {
        Self {
            id,
            items,
            operation,
            divisible,
            id_true,
            id_false,
            inspected,
        }
    }
}

// Struct that holds game itself.
#[derive(Debug, Clone)]
struct KeepAway {
    // List of all Monkeys that will play the game.
    monkeys: Vec<Monkey>,
}

impl KeepAway {
    fn new(monkeys: Vec<Monkey>) -> Self {
        Self { monkeys }
    }

    // Execute one game round.
    fn round(&mut self) {
        // For each monkey in the Monkey list.
        for i in 0..self.monkeys.len() {
            // Get copy of the monkeys List.
            let mut monkeys_copy = self.monkeys.clone();
            // Used copied list to get required information about current monkey.
            let monkey = monkeys_copy.get_mut(i).unwrap();

            // For each item inside of monkey item inventory.
            for k in 0..monkey.items.len() {
                // Get current item.
                let item = monkey.items.get(k).unwrap();

                // Calculate new worry value by doing operation specified in the monkey struct.
                let new_value = monkey.operation.execute(*item);
                // Reduce worry level by required amount.
                let new_value = (new_value as f64 / 3.0).floor() as u64;

                // Id of the monkey we will throw this item to.
                let id_throw: usize;
                // Do the div test.
                if new_value % monkey.divisible == 0 {
                    // If passed -> id of throw to monkey is current monkey id_true field.
                    id_throw = monkey.id_true;
                } else {
                    // Else it's id_false field.
                    id_throw = monkey.id_false;
                }

                // Add this new item to the inventory of id_throw monkey.
                self.monkeys
                    .get_mut(id_throw)
                    .unwrap()
                    .items
                    .push(new_value);
            }

            // Now we will modify current monkey we have worked with.
            let mut current_monkey = self.monkeys.get_mut(i).unwrap();
            // All items we have worked with right now were inspected by this monkey, so we increase monkey inspected counter.
            current_monkey.inspected += monkey.items.len();
            // And all items were thrown to other monkeys, so we clear this monkey inventory before moving to a next one.
            current_monkey.items.clear();
        }
    }

    // Method that is used to calculate final puzzle score.
    fn monkey_business(&self) -> usize {
        // Print debugging string.
        self.print_inspected();

        // Collect every monkey inspected field into a List.
        let mut inspections: Vec<usize> = self.monkeys.iter().map(|m| m.inspected).collect();
        // Sort this list.
        inspections.sort_unstable();
        // Reverse it, so top 2 values are in the top of the list.
        inspections.reverse();

        // Caclulate final score.
        inspections[0] * inspections[1]
    }

    // Debugging string.
    fn print_inspected(&self) {
        for monkey in self.monkeys.iter() {
            println!(
                "Monkey {} inspected items {} times.",
                monkey.id, monkey.inspected
            );
        }
    }
}

// Nom stuff bellow. Check other solutions to get a better perspective on what nom parsers do.

// Nom parser to parse monkey id.
fn parse_monkey_id(i: &str) -> IResult<&str, usize> {
    let (i, id) = terminated(
        delimited(
            tag("Monkey "),
            nom::character::complete::u32,
            nom::character::complete::char(':'),
        ),
        newline,
    )(i)?;

    Ok((i, id as usize))
}

// Nom parser to parse monkey inventory.
fn parse_monkey_items(i: &str) -> IResult<&str, Vec<u64>> {
    let (i, items) = delimited(
        tag("  Starting items: "),
        separated_list1(tag(", "), nom::character::complete::u64),
        newline,
    )(i)?;

    Ok((i, items))
}

// Nom parser to parse monkey operations.
fn parse_monkey_operation(i: &str) -> IResult<&str, Operation> {
    let (i, op_char) = terminated(
        alt((
            nom::character::complete::char('*'),
            nom::character::complete::char('+'),
        )),
        nom::character::complete::char(' '),
    )(i)?;

    let (i, rhs_str) = alt((tag("old"), digit1))(i)?;

    let operation = match (op_char, rhs_str) {
        ('+', "old") => Operation::new(add, Operand::Old),
        ('+', s) => Operation::new(add, Operand::Number(u64::from_str_radix(s, 10).unwrap())),
        ('*', "old") => Operation::new(mul, Operand::Old),
        ('*', s) => Operation::new(mul, Operand::Number(u64::from_str_radix(s, 10).unwrap())),
        _ => todo!("Invalid operation."),
    };

    Ok((i, operation))
}

// Nom parser to parse monkey operations line.
fn parse_monkey_operation_line(i: &str) -> IResult<&str, Operation> {
    let (i, operation) = delimited(
        tag("  Operation: new = old "),
        parse_monkey_operation,
        newline,
    )(i)?;

    Ok((i, operation))
}

// Nom parser to parse monkey divisible field.
fn parse_monkey_divisible(i: &str) -> IResult<&str, u64> {
    let (i, divisible) = delimited(
        tag("  Test: divisible by "),
        nom::character::complete::u64,
        newline,
    )(i)?;

    Ok((i, divisible))
}

// Nom parser to parse monkey if true field.
fn parse_monkey_true(i: &str) -> IResult<&str, usize> {
    let (i, id) = delimited(
        tag("    If true: throw to monkey "),
        nom::character::complete::u32,
        newline,
    )(i)?;

    Ok((i, id as usize))
}

// Nom parser to parse monkey if false field.
fn parse_monkey_false(i: &str) -> IResult<&str, usize> {
    let (i, id) = preceded(
        tag("    If false: throw to monkey "),
        nom::character::complete::u32,
    )(i)?;

    Ok((i, id as usize))
}

// Nom parser to parse whole monkey struct.
fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
    let (i, id) = parse_monkey_id(i)?;
    let (i, items) = parse_monkey_items(i)?;
    let (i, operation) = parse_monkey_operation_line(i)?;
    let (i, divisible) = parse_monkey_divisible(i)?;
    let (i, id_true) = parse_monkey_true(i)?;
    let (i, id_false) = parse_monkey_false(i)?;

    Ok((
        i,
        Monkey::new(id, items, operation, divisible, id_true, id_false, 0),
    ))
}

// Nom parser to parse whole file.
fn parse_file(i: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(preceded(newline, newline), parse_monkey)(i)
}

// Main program entry.
fn main() -> Result<(), Box<dyn Error>> {
    // Read input file into String.
    let file = std::fs::read_to_string("../input.txt")?;
    // Get parsed List of all monkeys.
    let (_, monkeys) = parse_file(file.as_str()).unwrap();
    // Initiate new game with List of parsed monkeys.
    let mut game = KeepAway::new(monkeys);

    // Do 20 rounds.
    for _ in 0..20 {
        game.round();
    }

    // Calculate and print final score.
    println!("Monkey business: {}", game.monkey_business());

    Ok(())
}
