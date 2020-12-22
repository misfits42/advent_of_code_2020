use std::collections::VecDeque;

use regex::Regex;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Token {
    Operand { value: u64 },
    OperatorPlus,
    OperatorMult,
    ParenOpen,
    ParenClose,
}

impl Token {
    fn from_string(input: &str) -> Option<Token> {
        let num_regex = Regex::new(r"^\d+$").unwrap();
        let input = input.trim();
        if num_regex.is_match(input) {
            return Some(Token::Operand {
                value: input.parse::<u64>().unwrap(),
            });
        } else if input == "+" {
            return Some(Token::OperatorPlus);
        } else if input == "*" {
            return Some(Token::OperatorMult);
        } else if input == "(" {
            return Some(Token::ParenOpen);
        } else if input == ")" {
            return Some(Token::ParenClose);
        }
        panic!("Token - failed to parse input string!");
    }
}

#[aoc_generator(day18)]
fn generate_input(input: &str) -> Vec<Vec<Token>> {
    let mut expressions: Vec<Vec<Token>> = vec![];
    for line in input.lines() {
        let mut exp: Vec<Token> = vec![];
        // Trim leading and trailing whitespace, then ignore empty lines
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Ensure open and close parentheses are surrounded by whitespace
        let line = line.replace("(", " ( ").replace(")", " ) ");
        for raw_token in line.split_whitespace() {
            exp.push(Token::from_string(&raw_token).unwrap());
        }
        expressions.push(exp);
    }
    return expressions;
}

#[aoc(day18, part1)]
fn solve_part_1(expressions: &Vec<Vec<Token>>) -> u64 {
    let mut sum_result = 0;
    for exp in expressions {
        let mut index = 0;
        sum_result += evaluate_expression_recursive(exp, &mut index, 0);
    }
    return sum_result;
}

#[aoc(day18, part2)]
fn solve_part_2(expressions: &Vec<Vec<Token>>) -> u64 {
    let mut sum_result = 0;
    for exp in expressions {
        let exp_rpn = convert_exp_infix_to_rpn(exp);
        sum_result += evaluate_expression_rpn(&exp_rpn);
    }
    return sum_result;
}

/// Converts an expression in infix notation to Reverse Polish Notation (RPN) (postfix).
fn convert_exp_infix_to_rpn(exp: &Vec<Token>) -> Vec<Token> {
    let mut output: Vec<Token> = vec![];
    let mut op_stack: VecDeque<Token> = VecDeque::new();
    for token in exp.iter() {
        let token = *token;
        match token {
            Token::Operand { value: _ } => output.push(token),
            Token::OperatorPlus => op_stack.push_front(token),
            Token::OperatorMult => {
                while !op_stack.is_empty() && *op_stack.front().unwrap() == Token::OperatorPlus {
                    output.push(op_stack.pop_front().unwrap());
                }
                op_stack.push_front(token);
            }
            Token::ParenOpen => {
                op_stack.push_front(token);
            }
            Token::ParenClose => {
                while !op_stack.is_empty() && *op_stack.front().unwrap() != Token::ParenOpen {
                    output.push(op_stack.pop_front().unwrap());
                }
                op_stack.pop_front(); // Ignore the left paren
            }
        }
    }
    while !op_stack.is_empty() {
        output.push(op_stack.pop_front().unwrap());
    }
    return output;
}

/// Evaluates the given expression in Reverse Polish Notation (postfix)
fn evaluate_expression_rpn(exp_rpn: &Vec<Token>) -> u64 {
    let mut result_stack: VecDeque<u64> = VecDeque::new();
    for token in exp_rpn.iter() {
        let token = *token;
        match token {
            Token::Operand { value } => result_stack.push_front(value),
            Token::OperatorPlus => {
                let left = result_stack.pop_front().unwrap();
                let right = result_stack.pop_front().unwrap();
                let result = left + right;
                result_stack.push_front(result);
            }
            Token::OperatorMult => {
                let left = result_stack.pop_front().unwrap();
                let right = result_stack.pop_front().unwrap();
                let result = left * right;
                result_stack.push_front(result);
            }
            _ => (),
        }
    }
    return result_stack.pop_front().unwrap();
}

/// Evaluates the given expression in infix notation, assuming all operators have the same
/// precedence.
fn evaluate_expression_recursive(exp_infix: &Vec<Token>, index: &mut usize, depth: usize) -> u64 {
    let mut result = 0;
    let mut last_operator = Token::OperatorPlus;
    while *index < exp_infix.len() {
        let token = exp_infix[*index];
        match token {
            Token::Operand { value } => {
                if last_operator == Token::OperatorPlus {
                    result += value;
                } else if last_operator == Token::OperatorMult {
                    result *= value;
                }
            }
            Token::OperatorMult => last_operator = token,
            Token::OperatorPlus => last_operator = token,
            Token::ParenOpen => {
                *index += 1;
                let sub_result = evaluate_expression_recursive(exp_infix, index, depth + 1);
                if last_operator == Token::OperatorPlus {
                    result += sub_result;
                } else if last_operator == Token::OperatorMult {
                    result *= sub_result;
                }
            }
            Token::ParenClose => {
                return result;
            }
        }
        *index += 1;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d18_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day18.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(45283905029161, result);
    }

    #[test]
    fn test_d18_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day18.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(216975281211165, result);
    }

    #[test]
    fn test_d18_p1_001() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day18_test_001.txt").unwrap(),
        );
        let result = solve_part_1(&input);
        assert_eq!(71, result);
    }

    #[test]
    fn test_d18_p1_002() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day18_test_002.txt").unwrap(),
        );
        let result = solve_part_1(&input);
        assert_eq!(26, result);
    }

    #[test]
    fn test_d18_p1_003() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day18_test_003.txt").unwrap(),
        );
        let result = solve_part_1(&input);
        assert_eq!(437, result);
    }

    #[test]
    fn test_d18_p1_004() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day18_test_004.txt").unwrap(),
        );
        let result = solve_part_1(&input);
        assert_eq!(12240, result);
    }

    #[test]
    fn test_d18_p1_005() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day18_test_005.txt").unwrap(),
        );
        let result = solve_part_1(&input);
        assert_eq!(13632, result);
    }

    #[test]
    fn test_d18_p2_001() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day18_test_001.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(231, result);
    }

    #[test]
    fn test_d18_p2_002() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day18_test_002.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(46, result);
    }

    #[test]
    fn test_d18_p2_003() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day18_test_003.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(1445, result);
    }

    #[test]
    fn test_d18_p2_004() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day18_test_004.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(669060, result);
    }

    #[test]
    fn test_d18_p2_005() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day18_test_005.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(23340, result);
    }
}
