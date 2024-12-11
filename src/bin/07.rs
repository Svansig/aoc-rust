use std::collections::VecDeque;

// TODO: This is super slow, it was completely naiive because I was having an issue with the u32 that the template expected

advent_of_code::solution!(7);

#[derive(Clone, PartialEq)]
enum Expression {
    Value(usize),
    Operator(Operators),
}

#[derive(Clone, PartialEq)]
enum Operators {
    Add,
    Multiply,
    Concatenate,
}

impl Operators {
    fn get_char(&self) -> String {
        match self {
            Operators::Add => "+".to_string(),
            Operators::Multiply => "*".to_string(),
            Operators::Concatenate => "||".to_string(),
        }
    }

    fn apply(&self, a: usize, b: usize) -> usize {
        match self {
            Operators::Add => a + b,
            Operators::Multiply => a * b,
            Operators::Concatenate => {
                // This should work the same as a string concatenation
                // but for numbers
                let mut a = a;
                let mut b_ref = b;
                while b_ref > 0 {
                    a *= 10;
                    b_ref /= 10;
                }
                a + b
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct Solution {
    operators: Vec<Expression>,
}

impl Solution {
    fn add_expression(&mut self, expression: Expression) {
        self.operators.push(expression);
    }
    fn evaluate(&self) -> usize {
        // These are infix operators
        let mut ops: VecDeque<Expression> = self.operators.clone().into();
        let mut acc = match ops.pop_front().unwrap() {
            Expression::Value(x) => x,
            _ => panic!("First element must be a value"),
        };
        while let Some(expr) = ops.pop_front() {
            match expr {
                Expression::Operator(op) => {
                    acc = op.apply(
                        acc,
                        match ops.pop_front().unwrap() {
                            Expression::Value(x) => x,
                            _ => panic!("Value must follow operator"),
                        },
                    )
                }
                _ => panic!("Operator must follow value"),
            }
        }
        acc
    }

    fn print(&self) -> () {
        let mut result = Vec::new();
        result.push(self.evaluate().to_string());
        result.push("=".to_string());
        for operator in self.operators.iter() {
            match operator {
                Expression::Value(x) => result.push(x.to_string()),
                Expression::Operator(op) => result.push(op.get_char()),
            }
        }
        println!("{}", result.join(" "));
    }
}

struct Equation {
    result: usize,
    operators: Vec<Operators>,
    operands: Vec<usize>,
}

impl Equation {
    fn from_line(line: &str, operators: Vec<Operators>) -> Equation {
        // Split based on ':', the left side is the result, the right side is the operands
        let parts: Vec<&str> = line.split(":").collect();
        let result = parts[0].parse::<usize>().unwrap();
        let operands = parts[1]
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        Equation {
            result,
            operands,
            operators,
        }
    }

    fn get_solutions(&self) -> Vec<Solution> {
        // Apply the operands left to right
        // Try all possible combinations of operators
        // If the result is found, add it to the list
        let mut operands: VecDeque<usize> = self.operands.clone().into();
        let mut accumulators: Vec<Solution> = vec![Solution {
            operators: vec![Expression::Value(operands.pop_front().unwrap())],
        }];
        for operand in operands {
            let mut new_accumulators: Vec<Solution> = vec![];
            for acc in accumulators {
                for operator in self.operators.iter() {
                    let mut new_acc = acc.clone();
                    new_acc.add_expression(Expression::Operator(operator.clone()));
                    new_acc.add_expression(Expression::Value(operand));
                    new_accumulators.push(new_acc);
                }
            }
            accumulators = new_accumulators;
        }
        accumulators
    }

    fn is_solvable(&self) -> bool {
        // Check if the equation is solvable
        // If the result is in the operands, it is solvable
        let solutions = self.get_solutions();
        for solution in solutions {
            if solution.evaluate() == self.result {
                // solution.print();
                return true;
            }
        }
        false
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let operators = vec![Operators::Add, Operators::Multiply];
    let equations: Vec<Equation> = input
        .lines()
        .map(|x| Equation::from_line(x, operators.clone()))
        .collect();
    Some(
        equations
            .iter()
            .filter(|x| x.is_solvable())
            .map(|x| x.result)
            .sum::<usize>(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let operators = vec![Operators::Add, Operators::Multiply, Operators::Concatenate];
    let equations: Vec<Equation> = input
        .lines()
        .map(|x| Equation::from_line(x, operators.clone()))
        .collect();
    Some(
        equations
            .iter()
            .filter(|x| x.is_solvable())
            .map(|x| x.result)
            .sum::<usize>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(267566105056));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(116094961956019));
    }
}
