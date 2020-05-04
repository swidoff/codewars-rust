use std::collections::HashMap;

type Atom = (String, usize);
type Molecule = Vec<Atom>;

#[derive(Debug)]
pub struct ParseError {}

enum Token {
    Name(String),
    Number(String),
    OpenBracket(char),
    CloseBracket(char),
}

fn lex(s: &str) -> Result<Vec<Token>, ParseError> {
    let mut stack: Vec<Token> = Vec::new();
    for c in s.chars() {
        match c {
            c if c.is_alphabetic() && c.is_uppercase() =>
                stack.push(Token::Name(c.to_string())),
            c if c.is_alphabetic() && c.is_lowercase() =>
                if let Some(Token::Name(sym)) = stack.last_mut() {
                    sym.push(c)
                } else {
                    return Err(ParseError {});
                },
            c if c.is_numeric() =>
                if let Some(Token::Number(sym)) = stack.last_mut() {
                    sym.push(c)
                } else {
                    stack.push(Token::Number(c.to_string()));
                }
            '[' | '(' | '{' => stack.push(Token::OpenBracket(c)),
            ']' | ')' | '}' => stack.push(Token::CloseBracket(c)),
            _ => return Err(ParseError {})
        }
    }

    return Ok(stack);
}

enum Symbol {
    Atom(String, u32),
    OpenBracket(),
    CloseBracket(usize),
}

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    let tokens = lex(s)?;
    let mut stack: Vec<Symbol> = Vec::new();
    let mut brackets: Vec<(char, usize)> = Vec::new();
    for token in tokens.iter() {
        match token {
            Token::Name(sym) => stack.push(Symbol::Atom(sym.clone(), 1)),
            Token::Number(num) => {
                let v = num.parse::<u32>().map_err(|_| ParseError {})?;
                let i = match stack.last() {
                    Some(Symbol::Atom(_, _)) => stack.len() - 1,
                    Some(Symbol::CloseBracket(i)) => *i,
                    _ => return Err(ParseError {})
                };
                for token in stack[i..].iter_mut() {
                    if let Symbol::Atom(_, size) = token {
                        *size *= v
                    }
                }
            }
            Token::OpenBracket(c) => {
                stack.push(Symbol::OpenBracket());
                brackets.push((*c, stack.len() - 1));
            }
            Token::CloseBracket(c) => {
                match brackets.pop() {
                    Some(('[', i)) if *c == ']' => stack.push(Symbol::CloseBracket(i)),
                    Some(('(', i)) if *c == ')' => stack.push(Symbol::CloseBracket(i)),
                    Some(('{', i)) if *c == '}' => stack.push(Symbol::CloseBracket(i)),
                    _ => return Err(ParseError {})
                }
            }
        }
    }

    if !brackets.is_empty() {
        return Err(ParseError {});
    }

    let mut counts: HashMap<&String, u32> = HashMap::new();
    let mut names: Vec<&String> = Vec::new();
    for token in stack.iter() {
        if let Symbol::Atom(sym, count) = token {
            match counts.get_mut(sym) {
                Some(total) => *total += count,
                None => {
                    counts.insert(sym, *count);
                    names.push(sym);
                }
            }
        }
    }

    Ok(names.into_iter().map(|sym| (sym.clone(), *counts.get(sym).unwrap() as usize)).collect::<Vec<(String, usize)>>())
}

#[test]
fn molecules() {
    assert_eq!(parse_molecule("H").unwrap(), vec![("H".to_string(), 1)]);
    assert_eq!(parse_molecule("O2").unwrap(), vec![("O".to_string(), 2)]);
    assert_eq!(parse_molecule("H2O").unwrap(), vec![("H".to_string(), 2), ("O".to_string(), 1)]);
    assert_eq!(parse_molecule("Mg(OH)2").unwrap(), vec![("Mg".to_string(), 1), ("O".to_string(), 2), ("H".to_string(), 2)]);
    assert_eq!(parse_molecule("C6H12O6").unwrap(), vec![("C".to_string(), 6), ("H".to_string(), 12), ("O".to_string(), 6)]);
    assert_eq!(parse_molecule("K4[ON(SO3)2]2").unwrap(), vec![("K".to_string(), 4), ("O".to_string(), 14), ("N".to_string(), 2), ("S".to_string(), 4)]);
    assert!(parse_molecule("pie").is_err());
    assert!(parse_molecule("Mg(OH").is_err());
    assert!(parse_molecule("Mg(OH}2").is_err());
}