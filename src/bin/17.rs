use std::fmt::Debug;

use itertools::Itertools;

advent_of_code::solution!(17);

#[derive(Clone)]
enum Instruction {
    ADV(Operand),
    BXL(Operand),
    BST(Operand),
    JNZ(Operand),
    BXC(Operand),
    OUT(Operand),
    BDV(Operand),
    CDV(Operand),
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::ADV(operand) => write!(f, "ADV {:?}", operand),
            Instruction::BXL(operand) => write!(f, "BXL {:?}", operand),
            Instruction::BST(operand) => write!(f, "BST {:?}", operand),
            Instruction::JNZ(operand) => write!(f, "JNZ {:?}", operand),
            Instruction::BXC(operand) => write!(f, "BXC {:?}", operand),
            Instruction::OUT(operand) => write!(f, "OUT {:?}", operand),
            Instruction::BDV(operand) => write!(f, "BDV {:?}", operand),
            Instruction::CDV(operand) => write!(f, "CDV {:?}", operand),
        }
    }
}

#[derive(Clone)]
enum Operand {
    Literal(usize),
    Combo(usize),
}

impl Debug for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Literal(value) => write!(f, "L({})", value),
            Operand::Combo(value) => write!(f, "C({})", value),
        }
    }
}

impl Operand {
    fn get_value(&self, state: &ComputerState) -> usize {
        match self {
            Operand::Literal(value) => *value,
            Operand::Combo(register) => match register {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => state.register_a.value,
                5 => state.register_b.value,
                6 => state.register_c.value,
                _ => panic!("Invalid Combo Value"),
            },
        }
    }

    fn literal_from_input(value: usize) -> Self {
        Operand::Literal(value)
    }

    fn combo_from_input(value: usize) -> Self {
        Operand::Combo(value)
    }
}

impl Instruction {
    fn from_input(opcode: usize, value: usize) -> Self {
        match opcode {
            0 => Instruction::ADV(Operand::combo_from_input(value)),
            1 => Instruction::BXL(Operand::literal_from_input(value)),
            2 => Instruction::BST(Operand::combo_from_input(value)),
            3 => Instruction::JNZ(Operand::literal_from_input(value)),
            4 => Instruction::BXC(Operand::literal_from_input(value)),
            5 => Instruction::OUT(Operand::combo_from_input(value)),
            6 => Instruction::BDV(Operand::combo_from_input(value)),
            7 => Instruction::CDV(Operand::combo_from_input(value)),
            _ => panic!("Invalid opcode"),
        }
    }

    fn perform_division<'a>(&self, state: &'a mut ComputerState) -> &'a mut ComputerState {
        let numerator = state.register_a.value;
        match self {
            Instruction::ADV(operand) => {
                let divisor = 2_usize.pow(operand.get_value(state) as u32);
                let quotient = numerator / divisor;
                state.register_a.value = quotient;
            }
            Instruction::BDV(operand) => {
                let divisor = 2_usize.pow(operand.get_value(state) as u32);
                let quotient = numerator / divisor;
                state.register_b.value = quotient;
            }
            Instruction::CDV(operand) => {
                let divisor = 2_usize.pow(operand.get_value(state) as u32);
                let quotient = numerator / divisor;
                state.register_c.value = quotient;
            }
            _ => panic!("Invalid instruction"),
        }

        state
    }

    fn get_operand_value(&self, state: &ComputerState) -> usize {
        match self {
            Instruction::ADV(operand) => operand.get_value(state),
            Instruction::BXL(operand) => operand.get_value(state),
            Instruction::BST(operand) => operand.get_value(state),
            Instruction::JNZ(operand) => operand.get_value(state),
            Instruction::BXC(operand) => operand.get_value(state),
            Instruction::OUT(operand) => operand.get_value(state),
            Instruction::BDV(operand) => operand.get_value(state),
            Instruction::CDV(operand) => operand.get_value(state),
        }
    }

    fn process<'a>(&self, mut state: &'a mut ComputerState) -> &'a mut ComputerState {
        // println!("{:?} - {}", self, self.get_operand_value(state));
        match self {
            Instruction::ADV(_operand) => {
                state = self.perform_division(state);
                state.instruction_pointer += 2;
            }
            Instruction::BXL(operand) => {
                state.register_b.value ^= operand.get_value(state);
                state.instruction_pointer += 2;
            }
            Instruction::BST(operand) => {
                let value = operand.get_value(state) % 8;
                state.register_b.value = value;
                state.instruction_pointer += 2;
            }
            Instruction::JNZ(operand) => {
                if state.register_a.value != 0 {
                    state.instruction_pointer = operand.get_value(state);
                } else {
                    state.instruction_pointer += 2;
                }
            }
            Instruction::BXC(_operand) => {
                state.register_b.value ^= state.register_c.value;
                state.instruction_pointer += 2;
            }
            Instruction::OUT(operand) => {
                state.output.push(operand.get_value(state) % 8);
                state.instruction_pointer += 2;
            }
            Instruction::BDV(_operand) => {
                state = self.perform_division(state);
                state.instruction_pointer += 2;
            }
            Instruction::CDV(_operand) => {
                state = self.perform_division(state);
                state.instruction_pointer += 2;
            }
        }

        state
    }
}

#[derive(Debug, Clone)]
struct Register {
    value: usize,
}

#[derive(Clone)]
struct ComputerState {
    register_a: Register,
    register_b: Register,
    register_c: Register,
    instruction_pointer: usize,
    output: Vec<usize>,
}

impl Debug for ComputerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComputerState")
            .field("A", &self.register_a.value)
            .field("B", &self.register_b.value)
            .field("C", &self.register_c.value)
            .field("IP", &self.instruction_pointer)
            .field("output", &self.output)
            .finish()
    }
}

#[derive(Clone)]
struct Computer {
    state: ComputerState,
    program_lines: String,
    instructions: Vec<Instruction>,
}

impl Computer {
    fn parse_from_input(input: &str) -> Self {
        let mut lines = input.lines();
        let register_a_line = lines.next().unwrap();
        let register_b_line = lines.next().unwrap();
        let register_c_line = lines.next().unwrap();
        let skip_line = lines.next().unwrap();
        let program_lines = lines.next().unwrap().split_whitespace().last().unwrap();

        let register_a = Register {
            value: register_a_line
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap(),
        };
        let register_b = Register {
            value: register_b_line
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap(),
        };
        let register_c = Register {
            value: register_c_line
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap(),
        };

        let mut instructions = Vec::new();

        for (opcode, value) in program_lines.split(',').tuple_windows() {
            instructions.push(Instruction::from_input(
                opcode.parse().unwrap(),
                value.parse().unwrap(),
            ));
        }

        Computer {
            state: ComputerState {
                register_a,
                register_b,
                register_c,
                instruction_pointer: 0,
                output: Vec::new(),
            },
            program_lines: program_lines.to_string(),
            instructions,
        }
    }

    fn run(&mut self) {
        while self.state.instruction_pointer < self.instructions.len() {
            let instruction = &self.instructions[self.state.instruction_pointer];
            instruction.process(&mut self.state);
        }
    }

    fn get_instructions(&self) -> String {
        self.program_lines.clone()
    }

    fn get_instructions_number(&self) -> Vec<usize> {
        self.program_lines
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect()
    }

    fn get_output_string(&self) -> String {
        self.state.output.iter().map(|x| x.to_string()).join(",")
    }

    fn get_output_number(&self) -> Vec<usize> {
        self.state.output.clone()
    }

    fn print_output(&self) {
        println!("{}", self.get_output_string());
    }

    fn print_output_with_instructions(&self) {
        println!("{} - {}", self.get_output_string(), self.get_instructions());
    }

    fn find_quine(&self) -> Option<usize> {
        let mut a_value = 0;
        let mut match_index = 0;
        let mut i = 0;
        loop {
            let test = a_value << 3 | i;
            let mut computer = self.clone();
            computer.state.register_a.value = test;
            computer.run();
            let output_values = computer.get_output_number();
            let program_lines = computer.get_instructions_number();
            // let output_match_index = output_values.len() - match_index - 1;
            // let test_match_index = program_lines.len() - match_index - 1;
            computer.print_output_with_instructions();

            // println!("---");
            // println!("[{}] {} - Checking at index: {}", i, test, test_match_index);
            // println!(
            //     "[{}] Expecting {} to match {}",
            //     i, output_values[output_match_index], program_lines[test_match_index]
            // );

            // Work backwards from the end and make sure everything at the end matches
            // If this exceeds the current match_index, then increment the match_index
            // Continue until no match

            let mut dec_match_index = 0;
            let mut exceeded_previous_match = false;
            while output_values[output_values.len() - 1 - dec_match_index]
                == program_lines[program_lines.len() - 1 - dec_match_index]
            {
                dec_match_index += 1;
                if dec_match_index >= program_lines.len() {
                    return Some(test);
                }
                if dec_match_index > match_index {
                    exceeded_previous_match = true;
                    match_index += 1;
                    i = 0;
                    a_value = test;
                }
                if dec_match_index >= output_values.len() {
                    break;
                }
            }
            if !exceeded_previous_match {
                i += 1;
            }
        }

        None
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer = Computer::parse_from_input(input);
    computer.run();
    Some(computer.get_output_string())
}

pub fn part_two(input: &str) -> Option<usize> {
    let computer = Computer::parse_from_input(input);
    computer.find_quine()
}

#[cfg(test)]
mod tests_17 {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
