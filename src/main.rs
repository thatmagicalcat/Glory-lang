use std::collections::HashMap;

use logos::Logos;
use rorth::Stack;
use rorth::Token;
use rorth::Token::*;

macro_rules! pop_back {
    ($s:ident) => {
        $s.pop_back().expect("Unexpected token")
    };
}

fn main() {
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();

        let tokens = rorth::Token::lexer(&buf);
        let tokens = tokens
            .map(|t| t.unwrap())
            .filter(|t| *t != Skip)
            .collect::<Vec<_>>();

        let cref = cross_reference(&tokens);
        if !cref.is_empty() {
            println!("{cref:#?}\n----");
        }

        let mut stack = Stack::new();
        let mut idx = 0;

        while idx < tokens.len() {
            let token = tokens[idx];

            // Everything consumes (pops the item it requires)
            match token {
                Number(n) => stack.push_back(n),

                Add => {
                    let v1 = pop_back!(stack);
                    let v2 = pop_back!(stack);

                    stack.push_back(v1 + v2);
                }

                Subtract => {
                    let v1 = pop_back!(stack);
                    let v2 = pop_back!(stack);

                    stack.push_back(v2 - v1);
                }

                Multiply => {
                    let v1 = pop_back!(stack);
                    let v2 = pop_back!(stack);

                    stack.push_back(v1 * v2);
                }

                Divide => {
                    let v1 = pop_back!(stack);
                    let v2 = pop_back!(stack);

                    stack.push_back(v2 / v1);
                }

                Lt => {
                    let v1 = pop_back!(stack);
                    let v2 = pop_back!(stack);

                    stack.push_back((v1 > v2) as _);
                }

                Gt => {
                    let v1 = pop_back!(stack);
                    let v2 = pop_back!(stack);

                    stack.push_back((v1 > v2) as _);
                }

                Eq => {
                    let v1 = pop_back!(stack);
                    let v2 = pop_back!(stack);

                    stack.push_back((v1 == v2) as _);
                }

                NEq => {
                    let v1 = pop_back!(stack);
                    let v2 = pop_back!(stack);

                    stack.push_back((v1 != v2) as _);
                }

                GtEq => {
                    let v1 = pop_back!(stack);
                    let v2 = pop_back!(stack);

                    stack.push_back((v2 >= v1) as _);
                }

                LtEq => {
                    let v1 = pop_back!(stack);
                    let v2 = pop_back!(stack);

                    stack.push_back((v2 <= v1) as _);
                }

                IF => {
                    let val = pop_back!(stack);

                    // go to end or else
                    if val == 0 {
                        let (else_pos, end_pos) = cref[&idx];
                        if let Some(else_pos) = else_pos {
                            idx = else_pos;
                        } else {
                            idx = end_pos;
                        }
                    }
                }

                ELSE => idx = cref[&idx].1 - 1, // -1 because it will get incremented at the end of the loop
                Clone => stack.push_back(stack.peek_back().expect("Unexpected token")),
                Print => print!("{}", pop_back!(stack)),
                Println => println!("{}", pop_back!(stack)),
                Clones(n) => {
                    if stack.len() < n as usize {
                        eprintln!("Error: insufficient items on the stack");
                        std::process::exit(1);
                    }

                    let slice = &stack.get_raw_stack().iter().copied().rev().collect::<Vec<_>>()[0..n as _];
                    slice.iter().rev().for_each(|i| stack.push_back(*i));
                }

                _ => {  }
            }

            idx += 1;
        }
    }
}

fn cross_reference(tokens: &[Token]) -> HashMap<usize, (Option<usize>, usize)> {
    let mut stack = vec![];
    let mut cref = HashMap::new();

    // If, Optional Else, End

    for (idx, token)in tokens.iter().enumerate() {
        if *token == IF {
            stack.push((idx, None))
        } else if *token == ELSE {
            stack.last_mut().expect("Else came before if").1 = Some(idx);
        } else if *token == END {
            let Some(entry) = stack.pop() else {
                panic!("Trailing end keyword");
            };

            cref.insert(entry.0, (entry.1, idx));

            // for getting the end keyword
            if let Some(else_pos) = entry.1 {
                cref.insert(else_pos, (None, idx));
            }
        }
    }

    if !stack.is_empty() { panic!("If without end keyword"); }

    cref
}
