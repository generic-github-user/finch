use std::env;
use std::fs;
use std::any::Any;
use std::mem;

#[derive(PartialEq, Clone)]
enum Node {
    Character(Character),
    Expr(Expr),
    Statement(Statement),
    Block
}

#[derive(PartialEq, Clone)]
enum Operation {
    UnaryOp { op: String, a: Expr },
    BinaryOp { op: String, a: Expr, b: Expr }
}

#[derive(PartialEq, Clone)]
enum Tuple {
    Default(Vec<Expr>)
}

#[derive(PartialEq, Clone)]
enum Expr {
    Any,
    Call { left: Box<Expr>, right: Box<Tuple> },
    Index,
    //Tuple(Vec<Expr>),
    Tuple(Tuple),
    Array,
    Operation,
    Literal,
    Identifier
}

#[derive(PartialEq, Clone)]
enum Literal {
    Float,
    Int,
    String
}

#[derive(PartialEq, Clone)]
enum Character {
    Any(char),
    Letter(char),
    Digit(char),
    Symbol(char)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let target = &args[1];
    println!("Parsing {}", target);
    let contents = fs::read_to_string(target)
        .expect("Encountered error while reading file");
    let alphabet: Vec<Node> = (b'A'..=b'z').map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .map(|c| Node::Character(Character::Any(c))).collect::<Vec<_>>();
    let digits: Vec<Node> = "0123456789".chars()
        .map(|c| Node::Character(Character::Any(c))).collect();
    let symbols: Vec<Node> = "!@#$%^&*()`~-_=+[]{};:,.<>/?|".chars()
        .map(|c| Node::Character(Character::Symbol(c))).collect();
}
