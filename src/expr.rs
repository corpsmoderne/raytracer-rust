use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Lpar,
    Rpar,
    Dot,
    Word(String)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Nil,
    Symbol(String),
    Num(i64),
    Float(f64),
    Cons(Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Nil => write!(f, "()"),
            Expr::Symbol(s) => write!(f, "{}", s),
            Expr::Num(i) => write!(f, "{}", i),
            Expr::Float(i) => write!(f, "{}", i),
            Expr::Cons(ref c1, ref c2) => {
                write!(f, "({}", *c1)?;
                fmt_cons(c2, f)                
            }
        }
    }
}

fn fmt_cons(mut cons: &Expr, f: &mut fmt::Formatter) -> fmt::Result {
    loop {
        match cons {
            Expr::Nil => break,
            Expr::Cons(ref cc1, ref cc2) => {
                write!(f, " {}", cc1)?;
                cons = cc2
            },
            exp => {
                write!(f, " . {}", exp)?;
                break
            }
        }       
    }
    write!(f, ")")
}

pub fn car(cons : &Expr) -> Expr {
    match cons {
        Expr::Cons(c1, _c2) => (**c1).clone(),
        _ => Expr::Nil
    }
}

pub fn cdr(cons : &Expr) -> Expr {
    match cons {
        Expr::Cons(_c1, c2) => (**c2).clone(),
        _ => Expr::Nil
    }
}

fn new_word(tok : &Vec<char>) -> Token {
    let s : String = tok.into_iter().collect();
    return Token::Word(s.to_string());
}

fn is_sep(c : char) -> bool {
    match c {
        '(' | ')' | ' ' | '\t' | '\n' | '\'' => true,
        _ => false
    } 
}

pub fn tokenize(input : &str) -> Vec<Token> {
    let mut vec : Vec<Token> = Vec::new();
    let mut tok : Vec<char> = Vec::new();

    for c in input.chars() {
        if is_sep(c) && tok.len() > 0 {
            vec.push(new_word(&tok));
            tok.clear();
        }
        match c {
            '(' => vec.push(Token::Lpar),
            ')' => vec.push(Token::Rpar),
            ' ' | '\t' | '\n' => {},
            _ => tok.push(c)
        }
    }
    if tok.len() > 0 {
        vec.push(new_word(&tok));
    }
    vec
}

fn parse_list(tokens : &[Token]) -> Option<(Expr , &[Token])> {
    if tokens.len() < 1 {
        return None
    }
    match &tokens[0] {
        Token::Rpar => Some((Expr::Nil, &tokens[1..])),
        Token::Dot => parse_expr(&tokens[1..])
            .and_then(|(cdr, rest)|
                      if rest.len() > 0 && rest[0] == Token::Rpar {
                          Some ((cdr, &rest[1..]))
                      } else {
                          None
                      }),
        _ => parse_expr(tokens)
            .and_then(|(car, rest)| 
                      parse_list(rest)
                      .and_then(|(cdr, rest2)|
                                Some((Expr::Cons(Box::new(car),
                                                 Box::new(cdr)), rest2))))
    }
}

pub fn parse_expr(tokens : &[Token]) -> Option<(Expr, &[Token])> {
    if tokens.len() < 1 {
        return None
    }
    match &tokens[0] {
        Token::Lpar => if tokens.len() < 2 || tokens[1] == Token::Dot {
            None
        } else {
            parse_list(&tokens[1..])
        },
        Token::Rpar => None, 
        Token::Dot => None,
        Token::Word(w) => Some (match w.parse::<i64>() {
            Ok(n) => (Expr::Num(n), &tokens[1..]),
            Err(_) => match w.parse::<f64>() {
                Ok(f) => (Expr::Float(f), &tokens[1..]),
                Err(_) => (Expr::Symbol(w.clone()), &tokens[1..])
            }
        })
    }
}

pub fn parse_all(tokens: &[Token]) -> Option<Vec<Expr>> {
    if tokens.len() == 0 {
        Some(vec![])        
    } else {
        parse_expr(tokens)
            .and_then(|(e1, rest)|
                      parse_all(rest)
                      .and_then( | e2 | Some([vec![e1], e2].concat())))
    }
}
