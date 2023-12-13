use std::iter::from_fn;
use std::str::from_utf8;
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

enum Token {
    LeftParen,
    RightParen,
    Number(i64),
    Plus,
    Star,
}

fn shunting_yard(line: &'static str, precedence_fn: impl Fn(&Token) -> usize) -> i64 {
    let mut operators: Vec<Token> = vec![];
    let mut output: Vec<Token> = vec![];

    for token in read_tokens(line) {
        match token {
            Token::LeftParen => operators.push(token),
            Token::RightParen => {
                while let Some(operator) = operators.last() {
                    if matches!(operator, Token::LeftParen) {
                        break;
                    }
                    output.push(operators.pop().unwrap());
                }

                if matches!(operators.last(), Some(Token::LeftParen)) {
                    operators.pop();
                }
            }
            Token::Number(_) => output.push(token),
            Token::Plus | Token::Star => {
                while let Some(operator) = operators.last() {
                    match operator {
                        Token::LeftParen | Token::RightParen => break,
                        Token::Plus | Token::Star => {
                            if precedence_fn(operator) >= precedence_fn(&token) {
                                output.push(operators.pop().unwrap());
                            } else {
                                break;
                            }
                        }
                        Token::Number(_) => unreachable!(),
                    }
                }
                operators.push(token);
            }
        }
    }

    output.extend(operators.drain(..).rev());

    rpn_calc(&output)
}

fn rpn_calc(tokens: &[Token]) -> i64 {
    let mut stack = vec![];

    for item in tokens {
        match item {
            Token::Number(n) => stack.push(*n),
            Token::Plus => {
                let result = stack.pop().unwrap() + stack.pop().unwrap();
                stack.push(result);
            }
            Token::Star => {
                let result = stack.pop().unwrap() * stack.pop().unwrap();
                stack.push(result);
            }
            _ => unreachable!(),
        };
    }

    assert_eq!(stack.len(), 1);
    stack[0]
}

fn read_tokens(line: &'static str) -> impl Iterator<Item = Token> {
    let bytes = line.as_bytes();
    let mut cur = 0;

    from_fn(move || {
        while let Some(char) = bytes.get(cur) {
            let mut result = None;

            match char {
                b'(' => result = Some(Token::LeftParen),
                b')' => result = Some(Token::RightParen),
                b'+' => result = Some(Token::Plus),
                b'*' => result = Some(Token::Star),
                c if c.is_ascii_digit() => {
                    let start = cur;
                    let mut end = start + 1;

                    while end < bytes.len() && bytes[end].is_ascii_digit() {
                        end += 1;
                    }

                    let num = from_utf8(&bytes[start..end]).unwrap().parse().unwrap();

                    result = Some(Token::Number(num));
                    cur = end - 1;
                }
                _ => (),
            }

            cur += 1;
            if result.is_some() {
                return result;
            }
        }

        None
    })
}

fn part1() -> i64 {
    let precedence_fn = |token: &Token| match token {
        Token::Plus | Token::Star => 0,
        _ => unreachable!(),
    };

    INPUT
        .lines()
        .map(|line| shunting_yard(line, precedence_fn))
        .sum()
}

fn part2() -> i64 {
    let precedence_fn = |token: &Token| match token {
        Token::Plus => 1,
        Token::Star => 0,
        _ => unreachable!(),
    };

    INPUT
        .lines()
        .map(|line| shunting_yard(line, precedence_fn))
        .sum()
}

fn main() {
    let start = Instant::now();
    println!("part 1: {}", part1());
    println!("part 1 took {}ms", start.elapsed().as_millis());
    let start = Instant::now();
    println!("part 2: {}", part2());
    println!("part 2 took {}ms", start.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 45840336521334);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 328920644404583);
    }
}
