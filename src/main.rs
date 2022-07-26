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
